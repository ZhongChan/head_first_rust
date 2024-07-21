use serde::{Deserialize, Serialize};

use super::question::QuestionId;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}
