use sqlx::PgPool;
use test_containers_util::sqlx_pg::PostgresTestDb;
use crate::modules::categories::CategoryRepositoryImpl;

static MIGRATIONS: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn create_test_db() -> PostgresTestDb {
    PostgresTestDb::create("categories_tests", &MIGRATIONS, None, None).await
}

pub fn pool_from_db(db: &PostgresTestDb) -> PgPool {
    db.pool()
}

pub fn create_repository(pool: PgPool) -> CategoryRepositoryImpl {
    CategoryRepositoryImpl::new(pool)
}