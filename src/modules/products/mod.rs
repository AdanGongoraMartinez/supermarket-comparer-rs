// Módulo de productos
pub mod controller;
pub mod repository;
pub mod repository_impl;
pub mod service;

#[cfg(test)]
pub mod tests;
pub mod types;

pub use controller::product_router;
pub use repository::ProductRepository;
pub use repository_impl::ProductRepositoryImpl;
pub use service::ProductService;
pub use types::{CreateProductInput, ProductSearchFilters};

