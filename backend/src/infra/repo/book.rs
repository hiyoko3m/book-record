use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
};
use sqlx::{postgres::PgPool, Row};

use crate::domain::entity::book::{BookEntity, BookEntityForCreation};
use crate::domain::entity::AxumError;
use crate::domain::repo_if::book::BookRepository;
use crate::infra::repo::schema::BookRow;

pub struct BookRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl<B> FromRequest<B> for BookRepositoryImpl
where
    B: Send,
{
    type Rejection = AxumError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<PgPool>::from_request(req)
            .await
            .map_err(|_| AxumError::PgConnectionError)?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn list_books(&self) -> Vec<BookEntity> {
        let rows = sqlx::query_as::<_, BookRow>("SELECT * FROM books ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                tracing::info!("cannot establish transaction: {}", err);
                ()
            });

        match rows {
            Ok(rows) => rows.into_iter().map(BookEntity::from).collect(),
            Err(_) => vec![],
        }
    }

    async fn get_book(&self, book_id: u32) -> Option<BookEntity> {
        sqlx::query_as::<_, BookRow>("SELECT * FROM books WHERE id = $1")
            .bind(book_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                // TODO
                // 存在しないIDをいれたときにもここでエラーログが出るのをどうにかしたい
                tracing::info!("cannot establish transaction: {}", err);
                ()
            })
            .map(BookEntity::from)
            .ok()
    }

    async fn create_book(&self, book: BookEntityForCreation) -> Result<u32, ()> {
        let mut transaction = self.pool.begin().await.map_err(|err| {
            tracing::info!("cannot establish transaction: {}", err);
            ()
        })?;

        let row = sqlx::query("INSERT INTO books (title) VALUES ($1) RETURNING id")
            .bind(book.title)
            .fetch_one(&mut transaction)
            .await
            .map_err(|err| {
                tracing::info!("insert was failed: {}", err);
                ()
            })?;

        transaction.commit().await.map_err(|err| {
            tracing::info!("commiting was failed: {}", err);
            ()
        })?;

        row.try_get::<i32, _>("id")
            // SQLの仕様ではsignedだが、値は0以上のものが返ってくる
            .map(|id| id as u32)
            .map_err(|err| {
                tracing::info!("parsing inserted id was failed: {}", err);
                ()
            })
    }

    async fn update_book(&self, book: BookEntity) -> bool {
        let inner = || async move {
            let mut transaction = self.pool.begin().await.map_err(|err| {
                tracing::info!("cannot establish transaction: {}", err);
                ()
            })?;

            let result = sqlx::query("UPDATE books SET title = $1 WHERE id = $2")
                .bind(book.title)
                .bind(book.id)
                .execute(&mut transaction)
                .await
                .map_err(|err| {
                    tracing::info!("update was failed: {}", err);
                    ()
                })?;

            transaction.commit().await.map_err(|err| {
                tracing::info!("commiting was failed: {}", err);
                ()
            })?;

            Ok(result.rows_affected() == 1)
        };

        let result: Result<bool, ()> = inner().await;
        if let Ok(result) = result {
            result
        } else {
            false
        }
    }

    async fn delete_book(&self, book_id: u32) -> bool {
        let inner = || async move {
            let mut transaction = self.pool.begin().await.map_err(|err| {
                tracing::info!("cannot establish transaction: {}", err);
                ()
            })?;

            let result = sqlx::query("DELETE FROM books WHERE id = $1")
                .bind(book_id)
                .execute(&mut transaction)
                .await
                .map_err(|err| {
                    tracing::info!("delete was failed: {}", err);
                    ()
                })?;

            transaction.commit().await.map_err(|err| {
                tracing::info!("commiting was failed: {}", err);
                ()
            })?;

            Ok(result.rows_affected() == 1)
        };

        let result: Result<bool, ()> = inner().await;
        if let Ok(result) = result {
            result
        } else {
            false
        }
    }
}
