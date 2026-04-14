// Entidad Producto
// Representa un producto en un supermercado

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub brand: Option<String>,
    pub presentation: Option<String>,
    pub barcode: Option<String>,
    pub category_id: Option<Uuid>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}
