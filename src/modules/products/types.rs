// Tipos para producto
// Input para crear y filtros para búsqueda

/// Input para crear un producto
#[derive(Debug, serde::Deserialize)]
pub struct CreateProductInput {
    pub name: String,
    pub brand: Option<String>,
    pub presentation: Option<String>,
    pub barcode: Option<String>,
    pub category_id: Option<String>,
}

/// Filtros para buscar productos
#[derive(Debug, serde::Deserialize)]
pub struct ProductSearchFilters {
    pub name: Option<String>,
    pub category_id: Option<String>,
    pub active_only: bool,
}
