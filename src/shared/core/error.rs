// Errores del dominio para productos
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProductError {
    #[error("Nombre de producto inválido: `{0}`")]
    InvalidName(String),

    #[error("El producto `{0}` ya existe")]
    AlreadyExists(String),

    #[error("Producto no encontrado: `{0}`")]
    NotFound(String),
}
