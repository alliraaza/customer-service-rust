use summer::plugin::service::Service;
use summer_sqlx::ConnectPool;
use crate::model::*;
use summer_sqlx::sqlx;
use crate::error::AppError;

#[derive(Clone, Service)]
pub struct CustomerService {
    #[inject(component)]
    db: ConnectPool,
}

impl CustomerService {
    pub async fn create(&self, req: CreateCustomer) -> Result<Customer, AppError> {
        sqlx::query_as::<_, Customer>(
            "INSERT INTO customers (name, email) VALUES ($1, $2) RETURNING id, name, email")
            .bind(req.name)
            .bind(req.email)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                // Postgres unique constraint violation -> 409 instead of 500
                sqlx::Error::Database(db) if db.is_unique_violation() => AppError::Conflict,
                other => AppError::Internal(other.into()),
            })
    }
    pub async fn get(&self, id: i64) -> Result<Customer, AppError> {
        sqlx::query_as::<_, Customer>("SELECT id, name, email FROM customers WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?   // DB failure -> 500
            .ok_or(AppError::NotFound(id))                // None -> 404
    }




    pub async fn list(&self, params: PageParams) -> Result<PageResponse<Customer>, AppError> {
        let limit = params.limit();
        let offset = params.offset();
        let items = sqlx::query_as::<_, Customer>("SELECT id, name, email FROM customers ORDER BY id ASC LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM customers")
            .fetch_one(&self.db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        Ok(PageResponse { items, page: params.page, size: limit, total })
    }

    pub async fn update(&self, id: i64, req: UpdateCustomer) -> Result<Customer, AppError> {
        let res = sqlx::query_as::<_, Customer>("UPDATE customers SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email")
            .bind(req.name)
            .bind(req.email)
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(db) if db.is_unique_violation() => AppError::Conflict,
                other => AppError::Internal(other.into()),
            })?;
        res.ok_or(AppError::NotFound(id))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        let rows = sqlx::query("DELETE FROM customers WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?
            .rows_affected();
        if rows == 0 { return Err(AppError::NotFound(id)); }
        Ok(())
    }

}