use serde::{Deserialize, Serialize};
use std::{fmt::*, io::Error, str::FromStr};

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub conent: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct QuestionId(pub String);

impl Question {
    pub fn new(id: QuestionId, title: String, conent: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            conent,
            tags,
        }
    }

    pub fn update_title(&self, new_title: String) -> Self {
        Question::new(
            self.id.clone(),
            new_title,
            self.conent.clone(),
            self.tags.clone(),
        )
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.conent, self.tags
        )
    }
}

impl Display for QuestionId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "id: {}", self.0)
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> std::result::Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "No id provieded",
            )),
        }
    }
}
