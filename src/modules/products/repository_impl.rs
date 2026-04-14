// Implementación del repository de producto usando SQLx

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::db::schema::ProductRow;
use crate::shared::core::{Result, ProductError};
use crate::shared::entities::Product;
use crate::modules::products::repository::ProductRepository;
use crate::modules::products::types::{CreateProductInput, ProductSearchFilters};

pub struct ProductRepositoryImpl {
    pool: PgPool,
}

impl Clone for ProductRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl ProductRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row_to_product(row: ProductRow) -> Product {
        Product {
            id: row.id,
            name: row.name,
            brand: row.brand,
            presentation: row.presentation,
            barcode: row.barcode,
            category_id: row.category_id,
            active: row.active,
            created_at: row.created_at,
        }
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    async fn create(&self, input: CreateProductInput) -> Result<Product, ProductError> {
        let category_id = input.category_id.as_ref().and_then(|s| Uuid::parse_str(s).ok());
        
        let row = sqlx::query_as::<_, ProductRow>(
            "INSERT INTO products (name, brand, presentation, barcode, category_id) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, name, brand, presentation, barcode, category_id, active, created_at"
        )
        .bind(&input.name)
        .bind(&input.brand)
        .bind(&input.presentation)
        .bind(&input.barcode)
        .bind(category_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ProductError::AlreadyExists(e.to_string()))?;

        Ok(Self::map_row_to_product(row))
    }

    async fn find_by_id(&self, id: &str) -> Result<Product, ProductError> {
        let row = sqlx::query_as::<_, ProductRow>(
            "SELECT id, name, brand, presentation, barcode, category_id, active, created_at 
             FROM products WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ProductError::NotFound(id.to_string()))?;

        Ok(Self::map_row_to_product(row))
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Product>, ProductError> {
        let rows = sqlx::query_as::<_, ProductRow>(
            "SELECT id, name, brand, presentation, barcode, category_id, active, created_at 
             FROM products WHERE name = $1"
        )
        .bind(name)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ProductError::NotFound(e.to_string()))?;

        Ok(rows.into_iter().map(Self::map_row_to_product).collect())
    }

    async fn search(&self, filters: ProductSearchFilters) -> Result<Vec<Product>, ProductError> {
        let name_pattern = filters.name.as_ref().map(|n| format!("%{}%", n));
        let category_id = filters.category_id.as_ref().and_then(|s| Uuid::parse_str(s).ok());
        
        let rows = sqlx::query_as::<_, ProductRow>(
            "SELECT id, name, brand, presentation, barcode, category_id, active, created_at 
             FROM products 
             WHERE ($1::bool IS NULL OR active = $1)
               AND ($2::text IS NULL OR name ILIKE $2)
               AND ($3::uuid IS NULL OR category_id = $3)"
        )
        .bind(filters.active_only)
        .bind(name_pattern)
        .bind(category_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ProductError::NotFound(e.to_string()))?;

        Ok(rows.into_iter().map(Self::map_row_to_product).collect())
    }

    async fn deactivate(&self, id: &str) -> Result<(), ProductError> {
        sqlx::query("UPDATE products SET active = false WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|_| ProductError::NotFound(id.to_string()))?;

        Ok(())
    }
}