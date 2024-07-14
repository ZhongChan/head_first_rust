use serde::{Deserialize, Serialize};

use super::question::QuestionId;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AnswerId(pub String);

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}
