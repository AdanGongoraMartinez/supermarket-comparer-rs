// Módulo de base de datos
// Conexión y queries a PostgreSQL usando SQLx

pub mod connection;
pub mod schema;

pub use connection::Db;
pub use schema::{CategoryRow, ProductRow};

