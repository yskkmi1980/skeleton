//! SQLite database layer (SQLx).

use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, Sqlite};

/// RFC3339 string to UTC timestamp parser.
#[allow(dead_code)]
pub(crate) fn parse_rfc3339_utc(s: &str) -> Result<DateTime<Utc>, sqlx::Error> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| sqlx::Error::Decode(format!("invalid RFC3339 datetime '{s}': {e}").into()))
}

#[derive(Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn init(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        // Initialize schema
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS system_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    /// Retrieve a system setting.
    pub async fn get_system_setting(&self, key: &str) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query("SELECT value FROM system_settings WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.get("value")))
    }

    /// Store or remove a system setting.
    pub async fn set_system_setting(
        &self,
        key: &str,
        value: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        if let Some(val) = value {
            let now = Utc::now().to_rfc3339();
            sqlx::query(
                "INSERT INTO system_settings (key, value, updated_at) VALUES (?, ?, ?)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
            )
            .bind(key)
            .bind(val)
            .bind(&now)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query("DELETE FROM system_settings WHERE key = ?")
                .bind(key)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_settings() {
        let db = Database::init("sqlite::memory:").await.unwrap();
        assert_eq!(db.get_system_setting("theme").await.unwrap(), None);

        db.set_system_setting("theme", Some("dark")).await.unwrap();
        assert_eq!(
            db.get_system_setting("theme").await.unwrap(),
            Some("dark".to_string())
        );

        db.set_system_setting("theme", None).await.unwrap();
        assert_eq!(db.get_system_setting("theme").await.unwrap(), None);
    }

    #[test]
    fn test_parse_rfc3339() {
        let valid = "2026-10-13T12:00:00Z";
        assert!(parse_rfc3339_utc(valid).is_ok());

        let invalid = "invalid-date";
        assert!(parse_rfc3339_utc(invalid).is_err());
    }
}
