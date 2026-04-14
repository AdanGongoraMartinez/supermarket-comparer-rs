// Service para categoría
// Lógica de negocio: validaciones, reglas del dominio

use crate::shared::core::{CategoryError, Result};
use crate::shared::entities::Category;
use crate::modules::categories::repository::CategoryRepository;
use crate::modules::categories::types::{CreateCategoryInput, CategorySearchFilters};

pub struct CategoryService<R: CategoryRepository> {
    repository: R,
}

impl<R: CategoryRepository> CategoryService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_category(
        &self,
        input: CreateCategoryInput,
    ) -> Result<Category, CategoryError> {
        if input.name.trim().is_empty() {
            return Err(CategoryError::InvalidName);
        }

        let existing: Vec<Category> = self.repository.find_by_name(&input.name).await?;
        if !existing.is_empty() {
            return Err(CategoryError::AlreadyExists(input.name));
        }

        self.repository.create(input).await
    }

    pub async fn get_category_by_id(&self, id: &str) -> Result<Category, CategoryError> {
        self.repository.find_by_id(id).await
    }

    pub async fn search_categories(
        &self,
        filters: CategorySearchFilters,
    ) -> Result<Vec<Category>, CategoryError> {
        self.repository.search(filters).await
    }

    pub async fn delete_category(&self, id: &str) -> Result<(), CategoryError> {
        self.repository.find_by_id(id).await?;
        self.repository.delete(id).await
    }
}