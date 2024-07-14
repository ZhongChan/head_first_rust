use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::*;
use std::sync::Arc;
use tokio::sync::RwLock;
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
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }
}

impl Store {
    pub fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../question.json");
        serde_json::from_str(file).expect("can't read question.json")
    }
}

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
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
            Error::QuestionNotFound => {
                write!(f, "Question not found")
            }
        }
    }
}

/// `marker trait`
/// `https://doc.rust-lang.org/std/marker/index.html`
/// `https://blog.rust-lang.org/2015/05/11/traits.html`
impl Reject for Error {}
