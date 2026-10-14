use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCustomer {
    #[validate(length(min = 1, max = 100, message = "name must be 1-100 chars"))]
    pub name: String,
    #[validate(email(message = "invalid email"))]
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct PageParams {
    #[serde(default)]
    pub page: i64,
    #[serde(default = "default_size")]
    pub size: i64,
}
fn default_size() -> i64 { 20 }

impl PageParams {
    pub fn limit(&self) -> i64 { self.size.clamp(1, 100) }   // cap it, always
    pub fn offset(&self) -> i64 { self.page.max(0) * self.limit() }
}



#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCustomer {
    #[validate(length(min = 1, max = 100, message = "name must be 1-100 chars"))]
    pub name: String,
    #[validate(email(message = "invalid email"))]
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub size: i64,
    pub total: i64,
}
