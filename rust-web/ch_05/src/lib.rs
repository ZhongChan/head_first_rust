use serde::{Deserialize, Serialize};
use std::fmt::*;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AnswerId(pub String);

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct QuestionId(pub String);

impl Display for Question {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl Display for QuestionId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "id: {}", self.0)
    }
}

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}
