// Schema de base de datos usando SQLx

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CategoryRow {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ProductRow {
    pub id: Uuid,
    pub name: String,
    pub brand: Option<String>,
    pub presentation: Option<String>,
    pub barcode: Option<String>,
    pub category_id: Option<Uuid>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}
