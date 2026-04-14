// Supermarket Comparer - API REST en Rust
// Exporta todos los módulos públicos

pub mod db;
pub mod modules;
pub mod shared;

pub use db::Db;
pub use shared::{core, entities};

