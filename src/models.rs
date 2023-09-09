use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionId {
    pub question_uuid: String
}

// ----------

// TODO: create a struct called `Answer`
//       derive the following traits: Serialize, Deserialize
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String

// TODO: create a struct called `AnswerDetail`
//       derive the following traits: Serialize, Deserialize, Debug, Clone, PartialEq
//       add a public `answer_uuid` field of type String
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String
//       add a public `created_at` field of type String

// TODO: create a struct called `AnswerId`
//       derive the following traits: Serialize, Deserialize
//       add a public `answer_uuid` field of type String
