#[cfg(test)]
mod products_tests {
    use crate::db::schema::ProductRow;
    use crate::shared::core::ProductError;
    use crate::shared::entities::Product;
    use sqlx::PgPool;
    use test_containers_util::sqlx_pg::PostgresTestDb;

    static MIGRATIONS: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

    async fn create_test_db() -> PostgresTestDb {
        PostgresTestDb::create("products_tests", &MIGRATIONS, None, None).await
    }

    fn pool_from_db(db: &PostgresTestDb) -> PgPool {
        db.pool()
    }

    async fn create_product(
        pool: &PgPool,
        name: &str,
        brand: Option<&str>,
        presentation: Option<&str>,
    ) -> Result<Product, ProductError> {
        let row = sqlx::query_as::<_, ProductRow>(
            "INSERT INTO products (name, brand, presentation) VALUES ($1, $2, $3) RETURNING id, name, brand, presentation, barcode, category_id, active, created_at"
        )
        .bind(name)
        .bind(brand)
        .bind(presentation)
        .fetch_one(pool)
        .await
        .map_err(|e| ProductError::AlreadyExists(e.to_string()))?;

        Ok(Product {
            id: row.id,
            name: row.name,
            brand: row.brand,
            presentation: row.presentation,
            barcode: row.barcode,
            category_id: row.category_id,
            active: row.active,
            created_at: row.created_at,
        })
    }

    async fn find_by_id(pool: &PgPool, id: &str) -> Result<Product, ProductError> {
        let row = sqlx::query_as::<_, ProductRow>(
            "SELECT id, name, brand, presentation, barcode, category_id, active, created_at FROM products WHERE id = $1"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|_| ProductError::NotFound(id.to_string()))?;

        Ok(Product {
            id: row.id,
            name: row.name,
            brand: row.brand,
            presentation: row.presentation,
            barcode: row.barcode,
            category_id: row.category_id,
            active: row.active,
            created_at: row.created_at,
        })
    }

    async fn search_products(
        pool: &PgPool,
        name_filter: Option<&str>,
        active_only: bool,
    ) -> Result<Vec<Product>, ProductError> {
        let query = if active_only {
            "SELECT id, name, brand, presentation, barcode, category_id, active, created_at FROM products WHERE name ILIKE $1 AND active = true"
        } else {
            "SELECT id, name, brand, presentation, barcode, category_id, active, created_at FROM products WHERE name ILIKE $1"
        };

        let rows = sqlx::query_as::<_, ProductRow>(query)
            .bind(name_filter.map(|n| format!("%{}%", n)).unwrap_or_default())
            .fetch_all(pool)
            .await
            .map_err(|e| ProductError::NotFound(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| Product {
                id: row.id,
                name: row.name,
                brand: row.brand,
                presentation: row.presentation,
                barcode: row.barcode,
                category_id: row.category_id,
                active: row.active,
                created_at: row.created_at,
            })
            .collect())
    }

    async fn deactivate_product(pool: &PgPool, id: &str) -> Result<(), ProductError> {
        sqlx::query("UPDATE products SET active = false WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| ProductError::NotFound(id.to_string()))?;

        Ok(())
    }

    #[tokio::test]
    async fn test_create_product_success() {
        let db = create_test_db().await;
        let pool = pool_from_db(&db);

        let result = create_product(&pool, "Milk", Some("Pascual"), Some("1L")).await;

        assert!(result.is_ok());
        let product = result.unwrap();
        assert_eq!(product.name, "Milk");
        assert_eq!(product.brand, Some("Pascual".to_string()));
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let db = create_test_db().await;
        let pool = pool_from_db(&db);

        let result = find_by_id(&pool, "00000000-0000-0000-0000-000000000000").await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ProductError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_search_products() {
        let db = create_test_db().await;
        let pool = pool_from_db(&db);

        create_product(&pool, "Milk", Some("Pascual"), Some("1L"))
            .await
            .unwrap();
        create_product(&pool, "Bread", Some("Bimbo"), None)
            .await
            .unwrap();

        let result = search_products(&pool, Some("Milk"), false).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_deactivate_product() {
        let db = create_test_db().await;
        let pool = pool_from_db(&db);

        let product = create_product(&pool, "ToDeactivate", None, None)
            .await
            .unwrap();
        let result = deactivate_product(&pool, &product.id.to_string()).await;

        assert!(result.is_ok());

        let deactivated = find_by_id(&pool, &product.id.to_string()).await.unwrap();
        assert!(!deactivated.active);
    }
}

