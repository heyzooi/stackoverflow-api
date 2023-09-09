use async_trait::async_trait;
use rocket::futures::future::ok;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self {
            db
        }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(
            // QuestionDetail,
            "INSERT INTO questions (title, description) VALUES ($1, $2) RETURNING *",
            question.title,
            question.description
        ).fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        // Populate the QuestionDetail fields using `record`.
        Ok(QuestionDetail{
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(&question_uuid)
        .map_err(|_| DBError::InvalidUUID(question_uuid.clone()))?;

        let query = sqlx::query!("DELETE FROM questions WHERE question_uuid = $1", uuid)
        .execute(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        if query.rows_affected() > 0 {
            Ok(())
        } else {
            Err(DBError::InvalidUUID(question_uuid))
        }
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        // Iterate over `records` and map each record to a `QuestionDetail` type
        let questions = records.into_iter().map(|record| QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        }).collect();

        Ok(questions)
    }
}
