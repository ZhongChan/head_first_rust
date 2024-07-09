use std::collections::HashMap;

use ch_02::{Question, QuestionId};

pub struct Store {
    pub questions: HashMap<QuestionId, Question>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }
}

impl Store {
    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }
}
