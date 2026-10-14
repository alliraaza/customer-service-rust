use summer_web::{get, post, put, delete};
use summer_web::extractor::{Component, Json, Path, Query};
use summer_web::axum::http::StatusCode;
use crate::{model::*, service::CustomerService, error::AppError};
use summer_sqlx::{sqlx, ConnectPool};
use validator::Validate;


#[post("/api/v1/customers")]
async fn create(Component(svc): Component<CustomerService>, Json(req): Json<CreateCustomer>)
    -> std::result::Result<(StatusCode, Json<Customer>), AppError> {
    req.validate()?;                       // 400 if invalid
    let c = svc.create(req).await?;
    Ok((StatusCode::CREATED, Json(c)))     // 201
}

#[get("/api/v1/customers/{id}")]
async fn get_one(Component(svc): Component<CustomerService>, Path(id): Path<i64>)
    -> std::result::Result<Json<Customer>, AppError> {
    Ok(Json(svc.get(id).await?))
}

#[get("/health")]
async fn health(Component(pool): Component<ConnectPool>) -> std::result::Result<&'static str, AppError> {
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok("UP")
}
#[get("/api/v1/customers")]
async fn list(Component(svc): Component<CustomerService>, Query(params): Query<PageParams>)
    -> std::result::Result<Json<PageResponse<Customer>>, AppError> {
    Ok(Json(svc.list(params).await?))
}

#[put("/api/v1/customers/{id}")]
async fn update(Component(svc): Component<CustomerService>, Path(id): Path<i64>, Json(req): Json<UpdateCustomer>)
    -> std::result::Result<Json<Customer>, AppError> {
    req.validate()?;
    Ok(Json(svc.update(id, req).await?))
}

#[delete("/api/v1/customers/{id}")]
async fn delete(Component(svc): Component<CustomerService>, Path(id): Path<i64>)
    -> std::result::Result<StatusCode, AppError> {
    svc.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
