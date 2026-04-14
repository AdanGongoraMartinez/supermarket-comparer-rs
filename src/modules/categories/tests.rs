#[cfg(test)]
mod categories_tests {
    use crate::db::schema::CategoryRow;
    use crate::shared::core::CategoryError;
    use crate::shared::entities::Category;
    use sqlx::PgPool;
    use test_containers_util::sqlx_pg::PostgresTestDb;

    static MIGRATIONS: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

    async fn create_test_db() -> PostgresTestDb {
        PostgresTestDb::create("categories_tests", &MIGRATIONS, None, None).await
    }

    fn pool_from_db(db: &PostgresTestDb) -> PgPool {
        db.pool()
    }

    async fn create_category(pool: &PgPool, name: &str) -> Result<Category, CategoryError> {
        let row = sqlx::query_as::<_, CategoryRow>(
            "INSERT INTO categories (name) VALUES ($1) RETURNING id, name, created_at",
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .map_err(|e| CategoryError::AlreadyExists(e.to_string()))?;

        Ok(Category {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
        })
    }

    async fn find_by_id(pool: &PgPool, id: &str) -> Result<Category, CategoryError> {
        let row = sqlx::query_as::<_, CategoryRow>(
            "SELECT id, name, created_at FROM categories WHERE id = $1",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|_| CategoryError::NotFound(id.to_string()))?;

        Ok(Category {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
        })
    }

    async fn find_by_name(pool: &PgPool, name: &str) -> Result<Vec<Category>, CategoryError> {
        let rows = sqlx::query_as::<_, CategoryRow>(
            "SELECT id, name, created_at FROM categories WHERE name = $1",
        )
        .bind(name)
        .fetch_all(pool)
        .await
        .map_err(|e| CategoryError::NotFound(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| Category {
                id: row.id,
                name: row.name,
                created_at: row.created_at,
            })
            .collect())
    }

    async fn search_categories(
        pool: &PgPool,
        name_filter: Option<&str>,
    ) -> Result<Vec<Category>, CategoryError> {
        let query = "SELECT id, name, created_at FROM categories WHERE name ILIKE $1";
        let rows = sqlx::query_as::<_, CategoryRow>(query)
            .bind(name_filter.map(|n| format!("%{}%", n)).unwrap_or_default())
            .fetch_all(pool)
            .await
            .map_err(|e| CategoryError::NotFound(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| Category {
                id: row.id,
                name: row.name,
                created_at: row.created_at,
            })
            .collect())
    }

    async fn delete_category(pool: &PgPool, id: &str) -> Result<(), CategoryError> {
        sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| CategoryError::NotFound(id.to_string()))?;

        Ok(())
    }

    #[tokio::test]
    async fn test_create_category_success() {
        let db = create_test_db().await;
        let pool = pool_from_db(&db);

        let result = create_category(&pool, "Electronics").await;

        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Electronics");
        assert!(!category.id.to_string().is_empty());
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let db = create_test_db().await;
        let pool = pool_from_db(&db);

        let result = find_by_id(&pool, "00000000-0000-0000-0000-000000000000").await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CategoryError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_find_by_name() {
        let db = create_test_db().await;
        let pool = pool_from_db(&db);

        create_category(&pool, "Food").await.unwrap();
        let result = find_by_name(&pool, "Food").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_search_categories() {
        let db = create_test_db().await;
        let pool = pool_from_db(&db);

        create_category(&pool, "Electronics").await.unwrap();
        create_category(&pool, "Food").await.unwrap();
        let result = search_categories(&pool, Some("Elec")).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_delete_category() {
        let db = create_test_db().await;
        let pool = pool_from_db(&db);

        let category = create_category(&pool, "ToDelete").await.unwrap();
        let result = delete_category(&pool, &category.id.to_string()).await;

        assert!(result.is_ok());

        let deleted = find_by_id(&pool, &category.id.to_string()).await;
        assert!(deleted.is_err());
    }
}

