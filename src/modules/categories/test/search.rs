use crate::modules::categories::{types::{CreateCategoryInput, CategorySearchFilters}, repository::CategoryRepository};
use super::test_helper::{create_test_db, pool_from_db, create_repository};

#[tokio::test]
async fn test_find_by_name_success() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    let input = CreateCategoryInput {
        name: "Food".to_string(),
    };
    repo.create(input).await.unwrap();
    let result = repo.find_by_name("Food").await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn test_find_by_name_not_found() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    let result = repo.find_by_name("NonExistent").await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_search_categories_success() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    repo.create(CreateCategoryInput {
        name: "Electronics".to_string(),
    }).await.unwrap();
    repo.create(CreateCategoryInput {
        name: "Food".to_string(),
    }).await.unwrap();
    
    let filters = CategorySearchFilters {
        name: Some("Elec".to_string()),
    };
    let result = repo.search(filters).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn test_search_categories_empty() {
    let db = create_test_db().await;
    let pool = pool_from_db(&db);
    let repo = create_repository(pool);

    repo.create(CreateCategoryInput {
        name: "Food".to_string(),
    }).await.unwrap();
    
    let filters = CategorySearchFilters {
        name: Some("XYZ".to_string()),
    };
    let result = repo.search(filters).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}