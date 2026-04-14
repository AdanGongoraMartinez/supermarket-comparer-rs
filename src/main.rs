// Supermarket Comparer - Entry Point

use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use supermarket_comparer_rs::db::Db;
use supermarket_comparer_rs::modules::categories::{
    category_router, CategoryRepositoryImpl, CategoryService,
};
use supermarket_comparer_rs::modules::products::{
    product_router, ProductRepositoryImpl, ProductService,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().expect(".env file not found");

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL no está configurada en el .env");

    println!("Connecting to database...");
    println!("{}", &database_url);

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

