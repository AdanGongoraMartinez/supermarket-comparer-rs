// Core modules re-exports
pub use std::result::Result;
pub mod api_response;
pub mod category_error;
pub mod error;
pub mod uuid;

pub use api_response::ApiResponse;
pub use category_error::CategoryError;
pub use error::ProductError;
pub use uuid::is_valid_uuid;
