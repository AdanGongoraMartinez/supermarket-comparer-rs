// Módulo de productos
pub mod types;
pub mod repository;
pub mod repository_impl;
pub mod service;
pub mod controller;

pub use types::{CreateProductInput, ProductSearchFilters};
pub use repository::ProductRepository;
pub use repository_impl::ProductRepositoryImpl;
pub use service::ProductService;
pub use controller::product_router;