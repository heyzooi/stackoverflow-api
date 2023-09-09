use chrono::Utc;
use rocket::{serde::json::Json};
use uuid::Uuid;

use crate::models::*;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
) -> Json<QuestionDetail> {
    let question = QuestionDetail {
        question_uuid: Uuid::new_v4().to_string(),
        title: question.title.clone(),
        description: question.description.clone(),
        created_at: Utc::now().to_rfc3339(),
    };
    Json(question)
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    let questions = vec![
        QuestionDetail {
            question_uuid: Uuid::new_v4().to_string(),
            title: "Question 1".to_owned(),
            description: "Question Desc 1".to_owned(),
            created_at: Utc::now().to_rfc3339(),
        },
        QuestionDetail {
            question_uuid: Uuid::new_v4().to_string(),
            title: "Question 1".to_owned(),
            description: "Question Desc 1".to_owned(),
            created_at: Utc::now().to_rfc3339(),
        }
    ];
    Json(questions)
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>
) {
    println!("Delete question: {question_uuid:?}");
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
) -> Json<AnswerDetail> {
    let answer = AnswerDetail {
        answer_uuid: Uuid::new_v4().to_string(),
        question_uuid: answer.question_uuid.clone(),
        content: answer.content.clone(),
        created_at: Utc::now().to_rfc3339(),
    };
    Json(answer)
}

#[get("/answers")]
pub async fn read_answers() -> Json<Vec<AnswerDetail>> {
    let answers = vec![
        AnswerDetail {
            answer_uuid: Uuid::new_v4().to_string(),
            question_uuid: Uuid::new_v4().to_string(),
            content: "Answer 1".to_owned(),
            created_at: Utc::now().to_rfc3339(),
        },
        AnswerDetail {
            answer_uuid: Uuid::new_v4().to_string(),
            question_uuid: Uuid::new_v4().to_string(),
            content: "Answer 2".to_owned(),
            created_at: Utc::now().to_rfc3339(),
        }
    ];
    Json(answers)
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>
) {
    println!("Delete answer: {answer_uuid:?}");
}
