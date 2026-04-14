// Módulo de base de datos
// Conexión y queries a PostgreSQL usando SQLx

pub mod schema;
pub mod connection;

pub use schema::{CategoryRow, ProductRow, CREATE_CATEGORIES_SQL, CREATE_PRODUCTS_SQL};
pub use connection::Db;