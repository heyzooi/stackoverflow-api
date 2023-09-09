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

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//       
//       hint: this function should look very similar to the create_question function above

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//       
//       hint: this function should look very similar to the read_questions function above

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//       
//       hint: this function should look very similar to the delete_question function above
