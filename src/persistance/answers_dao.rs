use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self {
            db
        }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = Uuid::parse_str(&answer.question_uuid)
        .map_err(|_| DBError::InvalidUUID(answer.question_uuid))?;

        let record = sqlx::query!(
            "
            INSERT INTO answers ( question_uuid, content )
            VALUES ( $1, $2 )
            RETURNING *
            ",
            uuid,
            answer.content
        ).fetch_one(&self.db)
        .await
        .map_err(|e| {
            e.as_database_error().and_then(|e| {
                e.code().and_then(|code| {
                    if code == postgres_error_codes::FOREIGN_KEY_VIOLATION {
                        Some(DBError::InvalidUUID(uuid.to_string()))
                    } else {
                        None
                    }
                })
            }).unwrap_or(DBError::Other(Box::new(e)))
        })?;

        // Populate the AnswerDetail fields using `record`.
        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(&answer_uuid)
        .map_err(|_| DBError::InvalidUUID(answer_uuid.clone()))?;

        let query = sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
        .execute(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        if query.rows_affected() == 1 {
            Ok(())
        } else {
            Err(DBError::InvalidUUID(answer_uuid))
        }
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `question_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = Uuid::parse_str(&question_uuid)
        .map_err(|_| DBError::InvalidUUID(question_uuid.clone()))?;

        // Make a database query to get all answers associated with a question uuid.
        // Here is the SQL query:
        // ```
        // SELECT * FROM answers WHERE question_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.
        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
        .fetch_all(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        // Iterate over `records` and map each record to a `AnswerDetail` type
        let answers = records.into_iter().map(|record| {
            AnswerDetail {
                answer_uuid: record.answer_uuid.to_string(),
                question_uuid: record.question_uuid.to_string(),
                content: record.content,
                created_at: record.created_at.to_string(),
            }
        }).collect();

        Ok(answers)
    }
}
