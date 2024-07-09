use ch_02::{Question, QuestionId};
use std::collections::HashMap;

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
