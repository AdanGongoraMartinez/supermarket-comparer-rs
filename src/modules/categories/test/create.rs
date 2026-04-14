use crate::modules::categories::{types::CreateCategoryInput, repository::CategoryRepository};
use crate::shared::core::CategoryError;
use super::test_helper::{create_test_db, pool_from_db, create_repository};

#[tokio::test]
async fn test_create_category_success() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    let input = CreateCategoryInput {
        name: "Electronics".to_string(),
    };
    let result = repo.create(input).await;

    assert!(result.is_ok());
    let category = result.unwrap();
    assert_eq!(category.name, "Electronics");
    assert!(!category.id.to_string().is_empty());
}

#[tokio::test]
async fn test_create_category_empty_name() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    let input = CreateCategoryInput {
        name: "".to_string(),
    };
    let result = repo.create(input).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, CategoryError::AlreadyExists(_)));
}