# Supermarket Comparer RS

API REST para comparar precios de productos entre diferentes supermercados (versión Rust).

## Funcionalidad

El proyecto permite gestionar productos y categorías de supermercados, facilitando la comparación de precios. Próximamente incluirá:

- **Productos**: Crear, buscar, obtener y desactivar productos
- **Categorías**: Crear, buscar, obtener y eliminar categorías
- **Supermercados**: Gestión de diferentes cadenas de supermercados
- **Precios**: Registro y comparación de precios por producto/supermercado

## Requisitos

- **Docker** y **Docker Compose**
- **Rust** (1.70+) con cargo

## Estado de Implementación

### Implementado

- ✅ API REST con Axum
- ✅ CRUD de Productos (crear, buscar, obtener por ID, desactivar)
- ✅ CRUD de Categorías (crear, buscar, obtener por ID, eliminar)
- ✅ Base de datos PostgreSQL con SQLx
- ✅ Migraciones SQLx
- ✅ Configuración con Docker y docker-compose
- ✅ Variables de entorno con .env

### Pendiente

- ❌ Tests unitarios
- ❌ Módulo de Supermercados
- ❌ Módulo de Precios
- ❌ Módulo de comparación de precios
- ❌ Autenticación/Autorización

## Configuración

### 1. Variables de entorno

Copia `env.example` a `.env`:

```bash
cp env.example .env
```

Contenido de `env.example`:
```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_dev
```

### 2. Docker

Iniciar PostgreSQL con Docker Compose:

```bash
docker-compose up -d
```

Esto inicia:
- **db**: PostgreSQL en puerto 5432
- **app**: La aplicación Rust (opcional)

### 3. Migraciones

Las migraciones están en la carpeta `migrations/`. Se ejecutan automáticamente al iniciar la aplicación.

Para crear una nueva migración:

```bash
cargo sqlx migrate add nombre_de_la_migracion
```

## Comandos

```bash
# Iniciar el servidor (requiere PostgreSQL)
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
├── db/
│   ├── schema.rs              # Definiciones de tablas
│   └── connection.rs          # Pool de conexiones
└── migrations/                # Migraciones SQL
```

Patrón: **Controller → Service → Repository**

## Tech Stack

- **Lenguaje**: Rust
- **Framework Web**: Axum 0.8
- **ORM**: SQLx
- **Database**: PostgreSQL (Docker)
- **Contenedores**: Docker Compose
- **Errores**: thiserror