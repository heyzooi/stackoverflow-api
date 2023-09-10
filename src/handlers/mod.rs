use chrono::Utc;
use rocket::{serde::json::Json, State};
use uuid::Uuid;

use crate::{models::*, persistance::{questions_dao::QuestionsDao, answers_dao::AnswersDao}};

mod handlers_inner;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
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
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Json<Vec<QuestionDetail>> {
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
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) {
    println!("Delete question: {question_uuid:?}");
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Json<AnswerDetail> {
    let answer = AnswerDetail {
        answer_uuid: Uuid::new_v4().to_string(),
        question_uuid: answer.question_uuid.clone(),
        content: answer.content.clone(),
        created_at: Utc::now().to_rfc3339(),
    };
    Json(answer)
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Json<Vec<AnswerDetail>> {
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
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) {
    println!("Delete answer: {answer_uuid:?}");
}
