// Implementación del repository de categoría usando SQLx

use async_trait::async_trait;
use std::sync::Arc;
use crate::db::Db;
use crate::db::schema::CategoryRow;
use crate::shared::core::{CategoryError, Result};
use crate::shared::entities::Category;
use crate::modules::categories::repository::CategoryRepository;
use crate::modules::categories::types::{CreateCategoryInput, CategorySearchFilters};

pub struct CategoryRepositoryImpl {
    db: Arc<Db>,
}

impl Clone for CategoryRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

impl CategoryRepositoryImpl {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    fn map_row_to_category(row: CategoryRow) -> Category {
        Category {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
        }
    }
}

#[async_trait]
impl CategoryRepository for CategoryRepositoryImpl {
    async fn create(&self, input: CreateCategoryInput) -> Result<Category, CategoryError> {
        let row = sqlx::query_as::<_, CategoryRow>(
            "INSERT INTO categories (name) VALUES ($1) RETURNING id, name, created_at"
        )
        .bind(&input.name)
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| CategoryError::AlreadyExists(e.to_string()))?;

        Ok(Self::map_row_to_category(row))
    }

    async fn find_by_id(&self, id: &str) -> Result<Category, CategoryError> {
        let row = sqlx::query_as::<_, CategoryRow>(
            "SELECT id, name, created_at FROM categories WHERE id = $1"
        )
        .bind(id)
        .fetch_one(self.db.pool())
        .await
        .map_err(|_| CategoryError::NotFound(id.to_string()))?;

        Ok(Self::map_row_to_category(row))
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Category>, CategoryError> {
        let rows = sqlx::query_as::<_, CategoryRow>(
            "SELECT id, name, created_at FROM categories WHERE name = $1"
        )
        .bind(name)
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| CategoryError::NotFound(e.to_string()))?;

        Ok(rows.into_iter().map(Self::map_row_to_category).collect())
    }

    async fn search(&self, filters: CategorySearchFilters) -> Result<Vec<Category>, CategoryError> {
        let query = "SELECT id, name, created_at FROM categories WHERE name ILIKE $1";
        let rows = sqlx::query_as::<_, CategoryRow>(query)
            .bind(filters.name.as_deref().map(|n| format!("%{}%", n)).unwrap_or_default())
            .fetch_all(self.db.pool())
            .await
            .map_err(|e| CategoryError::NotFound(e.to_string()))?;

        Ok(rows.into_iter().map(Self::map_row_to_category).collect())
    }

    async fn delete(&self, id: &str) -> Result<(), CategoryError> {
        sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(self.db.pool())
            .await
            .map_err(|_| CategoryError::NotFound(id.to_string()))?;

        Ok(())
    }
}