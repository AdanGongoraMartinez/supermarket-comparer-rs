// Service para producto

use crate::shared::core::{Result, ProductError};
use crate::shared::entities::Product;
use crate::modules::products::repository::ProductRepository;
use crate::modules::products::types::{CreateProductInput, ProductSearchFilters};

pub struct ProductService<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> ProductService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_product(
        &self,
        input: CreateProductInput,
    ) -> Result<Product, ProductError> {
        if input.name.trim().is_empty() {
            return Err(ProductError::InvalidName(input.name));
        }

        let existing: Vec<Product> = self.repository.find_by_name(&input.name).await?;
        let duplicated = existing.iter().any(|p| {
            p.name == input.name
                && p.brand == input.brand
                && p.presentation == input.presentation
        });

        if duplicated {
            return Err(ProductError::AlreadyExists(input.name));
        }

        self.repository.create(input).await
    }

    pub async fn get_product_by_id(&self, id: &str) -> Result<Product, ProductError> {
        self.repository.find_by_id(id).await
    }

    pub async fn search_products(
        &self,
        filters: ProductSearchFilters,
    ) -> Result<Vec<Product>, ProductError> {
        self.repository.search(filters).await
    }

    pub async fn deactivate_product(&self, id: &str) -> Result<(), ProductError> {
        self.repository.find_by_id(id).await?;
        self.repository.deactivate(id).await
    }
}