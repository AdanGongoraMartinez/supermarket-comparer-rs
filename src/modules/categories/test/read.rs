use crate::modules::categories::{types::CreateCategoryInput, repository::CategoryRepository};
use crate::shared::core::CategoryError;
use super::test_helper::{create_test_db, pool_from_db, create_repository};

#[tokio::test]
async fn test_find_by_id_success() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    let input = CreateCategoryInput {
        name: "Electronics".to_string(),
    };
    let category = repo.create(input).await.unwrap();
    let result = repo.find_by_id(&category.id.to_string()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "Electronics");
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    let result = repo.find_by_id("00000000-0000-0000-0000-000000000000").await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CategoryError::NotFound(_)));
}