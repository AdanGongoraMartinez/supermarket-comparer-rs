// Tipos para categoría
// Input para crear y filtros para búsqueda

/// Input para crear una categoría
#[derive(Debug, serde::Deserialize)]
pub struct CreateCategoryInput {
    pub name: String,
}

/// Filtros para buscar categorías
#[derive(Debug, serde::Deserialize)]
pub struct CategorySearchFilters {
    pub name: Option<String>,
}
