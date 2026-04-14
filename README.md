# Supermarket Comparer RS

API REST para comparar precios de productos entre diferentes supermercados (versión Rust).

## Funcionalidad

El proyecto permite gestionar productos y categorías de supermercados, facilitando la comparación de precios. Próximamente incluirá:

- **Productos**: Crear, buscar, obtener y desactivar productos
- **Categorías**: Crear, buscar, obtener y eliminar categorías
- **Supermercados**: Gestión de diferentes cadenas de supermercados
- **Precios**: Registro y comparación de precios por producto/supermercado

## Requisitos

- **Docker** y **Docker Compose** (para PostgreSQL)
- **Rust** (1.70+) con cargo

## Estado de Implementación

### Implementado

- ✅ API REST con Axum
- ✅ CRUD de Productos (crear, buscar, obtener por ID, desactivar)
- ✅ CRUD de Categorías (crear, buscar, obtener por ID, eliminar)
- ✅ Base de datos PostgreSQL con SQLx
- ✅ Patrón Result para manejo de errores

### Pendiente

- ❌ Tests unitarios sobre una base de datos secundaria (PRIORITARIO)
- ❌ Implementación de .env para manejo de variables de entorno (PRIORITARIO)
- ❌ Módulo de Supermercados
- ❌ Módulo de Precios
- ❌ Módulo de comparación de precios
- ❌ Migraciones automatizadas
- ❌ Autenticación/Autorización

## Instalación

1. **Clonar el repositorio**

2. **Configurar variables de entorno:**

   ```bash
   export DATABASE_URL="postgres://user:pass@localhost:5432/supermarket"
   ```

3. **Iniciar PostgreSQL:**

   ```bash
   docker run -d --name postgres \
     -e POSTGRES_USER=user \
     -e POSTGRES_PASSWORD=pass \
     -e POSTGRES_DB=supermarket \
     -p 5432:5432 postgres:15
   ```

4. **Compilar:**

   ```bash
   cargo build
   ```

## Comandos

```bash
# Iniciar el servidor
cargo run

# Verificar compilación
cargo check

# Tests
cargo test

# Compilar release
cargo build --release
```

## Ejemplos de uso de la API

### Health Check

```bash
curl http://localhost:3000/health
```

### Crear Categoría

```bash
curl -X POST http://localhost:3000/categories \
  -H "Content-Type: application/json" \
  -d '{"name": "Lácteos"}'
```

**Respuesta:**

```json
{
  "data": {
    "id": "uuid-generado",
    "name": "Lácteos",
    "created_at": "2024-01-01T00:00:00Z"
  },
  "error": null,
  "status": 201
}
```

### Crear Producto

```bash
curl -X POST http://localhost:3000/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Leche Entera",
    "brand": "Marca X",
    "presentation": "1L",
    "category_id": "uuid-de-categoria"
  }'
```

**Respuesta:**

```json
{
  "data": {
    "id": "uuid-generado",
    "name": "Leche Entera",
    "brand": "Marca X",
    "presentation": "1L",
    "category_id": "uuid-de-categoria",
    "active": true,
    "created_at": "2024-01-01T00:00:00Z"
  },
  "error": null,
  "status": 201
}
```

### Buscar Productos

```bash
curl "http://localhost:3000/products?name=Leche"
```

### Buscar Categorías

```bash
curl "http://localhost:3000/categories?name=Lácteos"
```

### Obtener Producto por ID

```bash
curl http://localhost:3000/products/{uuid}
```

### Obtener Categoría por ID

```bash
curl http://localhost:3000/categories/{uuid}
```

### Desactivar Producto

```bash
curl -X DELETE http://localhost:3000/products/{uuid}
```

### Eliminar Categoría

```bash
curl -X DELETE http://localhost:3000/categories/{uuid}
```

## Arquitectura

```
src/
├── main.rs                      # Entry point (Axum)
├── modules/
│   ├── categories/             # CRUD de categorías
│   │   ├── controller.rs
│   │   ├── service.rs
│   │   ├── repository.rs
│   │   ├── repository_impl.rs
│   │   └── types.rs
│   └── products/               # CRUD de productos
│       ├── controller.rs
│       ├── service.rs
│       ├── repository.rs
│       ├── repository_impl.rs
│       └── types.rs
├── shared/
│   ├── core/                  # Result, API Response, errores
│   └── entities/              # Entidades del dominio
└── db/
    ├── schema.rs              # Definiciones de tablas
    └── connection.rs         # Pool de conexiones
```

Patrón: **Controller → Service → Repository**

## Tech Stack

- **Lenguaje**: Rust
- **Framework Web**: Axum 0.7
- **ORM**: SQLx
- **Database**: PostgreSQL (Docker)
- **Errores**: thiserror

