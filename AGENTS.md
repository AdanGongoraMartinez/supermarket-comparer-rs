# Supermarket Comparer RS

## Commands

```bash
# Desarrollar
cargo run                          # Servidor en localhost:3000
cargo check                       # Verificar compilación
cargo test                        # Tests

# Base de datos (requiere PostgreSQL)
export DATABASE_URL="postgres://user:pass@localhost:5432/db"
```

## Arquitectura

- **Web**: Axum 0.7
- **DB**: SQLx (PostgreSQL)
- **Errores**: `thiserror` para errores de dominio

```
src/
├── main.rs          # Entry point
├── db/             # Schema + conexión
├── modules/        # Controller → Service → Repository
│   ├── categories/
│   └── products/
└── shared/         # Entities + errores
```

## Setup

1. PostgreSQL requerido (ver `docker-compose.yml` en proyecto TS)
2. Crear tablas con SQL en `src/db/schema.rs`

## Conventions

- Routes: `Controller` recibe `Service<R>` con estado. `Service` espera trait `Repository`.
- Repos deben implementar `Clone` (usar `Arc<Db>` interno).
- Errores usar `thiserror::Error` derive.
- Timestamps: `chrono::DateTime<Utc>`, IDs: `uuid::Uuid`.