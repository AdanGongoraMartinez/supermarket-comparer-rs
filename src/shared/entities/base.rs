// Entidad base con id y timestamps
// Todas las entidades heredan estos campos

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait BaseEntity {
    fn id(&self) -> Uuid;
    fn created_at(&self) -> DateTime<Utc>;
}
