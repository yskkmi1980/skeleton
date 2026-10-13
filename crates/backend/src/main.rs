use axum::{
    http::{header, Method},
    response::Html,
    routing::get,
    Router,
};
use backend::{build_api_router, AppState, Database};
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::info;

/// Application Server CLI options.
#[derive(Debug, Parser)]
#[command(name = "server", version, about = "Modular Application Backend Server")]
struct Cli {
    /// Port to listen on.
    #[arg(short, long, env = "PORT", default_value = "3000")]
    port: u16,

    /// Bind host address.
    #[arg(long, env = "HOST", default_value = "0.0.0.0")]
    host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=info,backend=info,tower_http=info".into()),
        )
        .init();

    info!("Starting Application Server...");

    // Initialize Database
    let db_path =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:app.db?mode=rwc".to_string());
    let db = Database::init(&db_path).await?;
    info!("Database initialized at {}", db_path);

    let state = Arc::new(AppState::new(Arc::new(db)));

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Build API router
    let api_routes = build_api_router(state);

    // Static files router (Frontend WASM build output)
    let frontend_dist = std::env::var("FRONTEND_DIST")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("crates/frontend/dist"));

    let app = if frontend_dist.exists() {
        info!("Serving frontend from: {:?}", frontend_dist);
        let index_file = frontend_dist.join("index.html");
        Router::new().nest("/api", api_routes).nest_service(
            "/",
            ServeDir::new(&frontend_dist).not_found_service(ServeFile::new(&index_file)),
        )
    } else {
        info!(
            "Frontend dist not found at {:?}, serving API only with fallback placeholder",
            frontend_dist
        );
        Router::new().nest("/api", api_routes).route(
            "/",
            get(|| async {
                Html("<h1>Application Backend API</h1><p>Visit /api/health for system status.</p>")
            }),
        )
    };

    let app = app.layer(cors).layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", cli.host, cli.port).parse()?;
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
