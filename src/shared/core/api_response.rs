// Modelo de respuesta API estándar
// Envuelve el resultado en un formato consistente

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub status: u16,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            status: 200,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            status: 201,
        }
    }

    pub fn no_content() -> Self {
        Self {
            data: None,
            error: None,
            status: 204,
        }
    }

    pub fn error(status: u16, message: String) -> Self {
        Self {
            data: None,
            error: Some(message),
            status,
        }
    }
}
