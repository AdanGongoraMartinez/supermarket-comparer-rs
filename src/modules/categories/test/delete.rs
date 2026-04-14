use crate::modules::categories::{types::CreateCategoryInput, repository::CategoryRepository};
use super::test_helper::{create_test_db, pool_from_db, create_repository};

#[tokio::test]
async fn test_delete_category_success() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    let input = CreateCategoryInput {
        name: "ToDelete".to_string(),
    };
    let category = repo.create(input).await.unwrap();
    let result = repo.delete(&category.id.to_string()).await;

    assert!(result.is_ok());

    let deleted = repo.find_by_id(&category.id.to_string()).await;
    assert!(deleted.is_err());
}

#[tokio::test]
async fn test_delete_category_not_found() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    let result = repo.delete("00000000-0000-0000-0000-000000000000").await;

    assert!(result.is_err());
}