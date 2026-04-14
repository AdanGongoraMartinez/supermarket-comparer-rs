// Supermarket Comparer - Entry Point

use std::sync::Arc;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use supermarket_comparer_rs::db::Db;
use supermarket_comparer_rs::modules::categories::{CategoryRepositoryImpl, CategoryService, category_router};
use supermarket_comparer_rs::modules::products::{ProductRepositoryImpl, ProductService, product_router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/supermarket".to_string());

    let db = Arc::new(Db::new(&database_url).await?);
    println!("✅ Connected to database");

    let category_repo = CategoryRepositoryImpl::new(Arc::clone(&db));
    let product_repo = ProductRepositoryImpl::new(Arc::clone(&db));

    let category_service = CategoryService::new(category_repo);
    let product_service = ProductService::new(product_repo);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(category_router(category_service))
        .merge(product_router(product_service))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server running on http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}