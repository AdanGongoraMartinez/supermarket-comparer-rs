// Repository para categoría.

use async_trait::async_trait;
use crate::shared::core::{CategoryError, Result};
use crate::shared::entities::Category;
use crate::modules::categories::types::{CreateCategoryInput, CategorySearchFilters};

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create(&self, input: CreateCategoryInput) -> Result<Category, CategoryError>;
    async fn find_by_id(&self, id: &str) -> Result<Category, CategoryError>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<Category>, CategoryError>;
    async fn search(&self, filters: CategorySearchFilters) -> Result<Vec<Category>, CategoryError>;
    async fn delete(&self, id: &str) -> Result<(), CategoryError>;
}