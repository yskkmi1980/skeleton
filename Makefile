.PHONY: help all release run dev test check clippy verify clean \
		frontend frontend-release frontend-serve \
		backend backend-release desktop run-desktop

export PATH := $(HOME)/.cargo/bin:$(PATH)

TRUNK_CONFIG := crates/frontend/Trunk.toml

# .env ファイルが存在すれば環境変数を読み込む（実行系ターゲット用）
ENV_FILE := .env
LOAD_ENV := if [ -f $(ENV_FILE) ]; then set -a; . ./$(ENV_FILE); set +a; fi;

help: ## 利用可能なコマンドを表示
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

all: release ## 全コンポーネントのリリースビルド（frontend + backend）

# ---------------------------------------------------------------------------
# ビルド
# ---------------------------------------------------------------------------

release: frontend-release backend-release ## 全リリースバイナリを一括ビルド

frontend: ## フロントエンドを開発ビルド (wasm)
	trunk --config $(TRUNK_CONFIG) build

frontend-release: ## フロントエンドをリリースビルド (crates/frontend/dist に出力)
	trunk --config $(TRUNK_CONFIG) build --release

frontend-serve: ## フロントエンド開発サーバーを起動 (ホットリロード)
	trunk --config $(TRUNK_CONFIG) serve --port 8080

backend: ## バックエンドを開発ビルド
	cargo build -p backend

backend-release: ## バックエンドをリリースビルド
	cargo build -p backend --release

desktop: frontend-release ## デスクトップアプリ（Tauri）をリリースビルド
	cargo build -p desktop --release

# ---------------------------------------------------------------------------
# 実行
# ---------------------------------------------------------------------------

run: release ## 全リリースビルド後、バックエンドを起動（.env を読み込み）
	$(LOAD_ENV) cargo run -p backend --release

dev: ## バックエンドを開発モードで起動（.env を読み込み）
	$(LOAD_ENV) cargo run -p backend

run-desktop: frontend ## デスクトップアプリを開発モードで起動（backend 起動必須、APP_BACKEND_URL で指定可）
	$(LOAD_ENV) cargo run -p desktop

# ---------------------------------------------------------------------------
# 品質チェック
# ---------------------------------------------------------------------------

check: ## 全クレートの品質チェック（fmt / clippy / wasm / test、check.sh と同一）
	./check.sh --check

test: ## 全クレートのテスト実行
	cargo test --workspace

clippy: ## 全クレートの静的コード解析
	cargo clippy --workspace --exclude frontend --all-targets -- -D warnings
	cargo clippy -p frontend --target wasm32-unknown-unknown -- -D warnings

verify: check frontend ## エージェント作業完了時の一括検証（fmt / clippy / wasm / test + frontend ビルド）

# ---------------------------------------------------------------------------
# クリーンアップ
# ---------------------------------------------------------------------------

clean: ## ビルド成果物をクリーンアップ
	cargo clean
	rm -rf crates/frontend/dist target

