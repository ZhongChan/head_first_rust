use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::*;

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

#[derive(Clone)]
pub struct Store {
    pub questions: HashMap<QuestionId, Question>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }
}

impl Store {
    pub fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../question.json");
        serde_json::from_str(file).expect("can't read question.json")
    }

    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }
}
