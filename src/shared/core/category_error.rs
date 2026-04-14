// Errores del dominio para categorías
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("Nombre de categoría inválido")]
    InvalidName,

    #[error("La categoría `{0}` ya existe")]
    AlreadyExists(String),

    #[error("Categoría no encontrada: `{0}`")]
    NotFound(String),
}
