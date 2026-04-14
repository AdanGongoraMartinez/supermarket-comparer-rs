// Controller HTTP para productos

use crate::modules::products::repository::ProductRepository;
use crate::modules::products::service::ProductService;
use crate::modules::products::types::{CreateProductInput, ProductSearchFilters};
use crate::shared::core::{is_valid_uuid, ApiResponse};
use crate::shared::entities::Product;
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct ProductQueryParams {
    pub name: Option<String>,
    pub category_id: Option<String>,
    pub active_only: Option<String>,
}

pub fn product_router<R>(service: ProductService<R>) -> Router
where
    R: ProductRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/products", post(create_product))
        .route("/products", get(list_products))
        .route("/products/{id}", get(get_product))
        .route("/products/{id}", delete(delete_product))
        .with_state(Arc::new(service))
}

async fn create_product<R>(
    State(service): State<Arc<ProductService<R>>>,
    Json(input): Json<CreateProductInput>,
) -> Json<ApiResponse<Product>>
where
    R: ProductRepository + Clone + Send + Sync + 'static,
{
    let result = service.create_product(input).await;
    Json(match result {
        Ok(product) => ApiResponse::created(product),
        Err(e) => ApiResponse::error(400, e.to_string()),
    })
}

async fn list_products<R>(
    State(service): State<Arc<ProductService<R>>>,
    Query(params): Query<ProductQueryParams>,
) -> Json<ApiResponse<Vec<Product>>>
where
    R: ProductRepository + Clone + Send + Sync + 'static,
{
    let filters = ProductSearchFilters {
        name: params.name.clone(),
        category_id: params.category_id.clone(),
        active_only: params
            .active_only
            .as_ref()
            .map(|s| s != "false")
            .unwrap_or(true),
    };
    let result = service.search_products(filters).await;
    Json(match result {
        Ok(products) => ApiResponse::ok(products),
        Err(e) => ApiResponse::error(404, e.to_string()),
    })
}

async fn get_product<R>(
    State(service): State<Arc<ProductService<R>>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Product>>
where
    R: ProductRepository + Clone + Send + Sync + 'static,
{
    if !is_valid_uuid(&id) {
        return Json(ApiResponse::error(400, "Invalid UUID".to_string()));
    }
    let result = service.get_product_by_id(&id).await;
    Json(match result {
        Ok(product) => ApiResponse::ok(product),
        Err(e) => ApiResponse::error(404, e.to_string()),
    })
}

async fn delete_product<R>(
    State(service): State<Arc<ProductService<R>>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<()>>
where
    R: ProductRepository + Clone + Send + Sync + 'static,
{
    if !is_valid_uuid(&id) {
        return Json(ApiResponse::error(400, "Invalid UUID".to_string()));
    }
    let result = service.deactivate_product(&id).await;
    Json(match result {
        Ok(_) => ApiResponse::no_content(),
        Err(e) => ApiResponse::error(404, e.to_string()),
    })
}
