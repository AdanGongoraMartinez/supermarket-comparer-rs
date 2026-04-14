// Controller HTTP para categorías

use std::sync::Arc;
use axum::{
    extract::{Path, State, Query},
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::Deserialize;
use crate::shared::core::{ApiResponse, Result, is_valid_uuid};
use crate::shared::entities::Category;
use crate::modules::categories::types::{CreateCategoryInput, CategorySearchFilters};
use crate::modules::categories::repository::CategoryRepository;
use crate::modules::categories::service::CategoryService;

#[derive(Deserialize, Default)]
pub struct CategoryQueryParams {
    pub name: Option<String>,
}

pub fn category_router<R>(service: CategoryService<R>) -> Router
where
    R: CategoryRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/categories", post(create_category))
        .route("/categories", get(list_categories))
        .route("/categories/:id", get(get_category))
        .route("/categories/:id", delete(delete_category))
        .with_state(Arc::new(service))
}

async fn create_category<R>(
    State(service): State<Arc<CategoryService<R>>>,
    Json(input): Json<CreateCategoryInput>,
) -> Json<ApiResponse<Category>>
where
    R: CategoryRepository + Clone + Send + Sync + 'static,
{
    let result = service.create_category(input).await;
    Json(match result {
        Ok(category) => ApiResponse::created(category),
        Err(e) => ApiResponse::error(400, e.to_string()),
    })
}

async fn list_categories<R>(
    State(service): State<Arc<CategoryService<R>>>,
    Query(params): Query<CategoryQueryParams>,
) -> Json<ApiResponse<Vec<Category>>>
where
    R: CategoryRepository + Clone + Send + Sync + 'static,
{
    let filters = CategorySearchFilters { name: params.name };
    let result = service.search_categories(filters).await;
    Json(match result {
        Ok(categories) => ApiResponse::ok(categories),
        Err(e) => ApiResponse::error(404, e.to_string()),
    })
}

async fn get_category<R>(
    State(service): State<Arc<CategoryService<R>>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Category>>
where
    R: CategoryRepository + Clone + Send + Sync + 'static,
{
    if !is_valid_uuid(&id) {
        return Json(ApiResponse::error(400, "Invalid UUID".to_string()));
    }
    
    let result = service.get_category_by_id(&id).await;
    Json(match result {
        Ok(category) => ApiResponse::ok(category),
        Err(e) => ApiResponse::error(404, e.to_string()),
    })
}

async fn delete_category<R>(
    State(service): State<Arc<CategoryService<R>>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<()>>
where
    R: CategoryRepository + Clone + Send + Sync + 'static,
{
    if !is_valid_uuid(&id) {
        return Json(ApiResponse::error(400, "Invalid UUID".to_string()));
    }
    
    let result = service.delete_category(&id).await;
    Json(match result {
        Ok(_) => ApiResponse::no_content(),
        Err(e) => ApiResponse::error(404, e.to_string()),
    })
}