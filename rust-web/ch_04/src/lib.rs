use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::*;
use warp::reject::Reject;

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

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::ParseError(err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => {
                write!(f, "Missing Parameter")
            }
        }
    }
}

/// `marker trait`
/// `https://doc.rust-lang.org/std/marker/index.html`
/// `https://blog.rust-lang.org/2015/05/11/traits.html`
impl Reject for Error {}
