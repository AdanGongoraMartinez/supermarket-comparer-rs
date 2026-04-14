// Conexión a PostgreSQL usando SQLx
// Pool de conexiones para reuse

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Pool de conexiones a PostgreSQL
pub struct Db {
    pool: PgPool,
}

impl Db {
    /// Crea un nuevo pool desde la URL de conexión
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Retorna el pool para usar en queries
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

// Implementa Drop para cerrar el pool al final
impl Drop for Db {
    fn drop(&mut self) {
        // Sqlx cierra automáticamente el pool
    }
}