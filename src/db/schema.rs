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

pub const CREATE_CATEGORIES_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
"#;

pub const CREATE_PRODUCTS_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    brand TEXT,
    presentation TEXT,
    barcode TEXT,
    category_id UUID REFERENCES categories(id),
    active BOOLEAN DEFAULT TRUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
"#;
