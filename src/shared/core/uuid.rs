// Validación de UUID
// Valida que un string sea un UUID válido

use uuid::Uuid;

pub fn is_valid_uuid(s: &str) -> bool {
    Uuid::parse_str(s).is_ok()
}
