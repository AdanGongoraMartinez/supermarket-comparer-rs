// Repository para producto.
// Contrato (trait) que define las operaciones de persistencia.

use async_trait::async_trait;
use crate::shared::core::{Result, ProductError};
use crate::shared::entities::Product;
use crate::modules::products::types::{CreateProductInput, ProductSearchFilters};

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, input: CreateProductInput) -> Result<Product, ProductError>;
    async fn find_by_id(&self, id: &str) -> Result<Product, ProductError>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<Product>, ProductError>;
    async fn search(&self, filters: ProductSearchFilters) -> Result<Vec<Product>, ProductError>;
    async fn deactivate(&self, id: &str) -> Result<(), ProductError>;
}