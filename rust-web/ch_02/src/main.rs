use std::{fmt::*, io::Error, str::FromStr};
mod tests;

#[derive(Clone)]
struct Question {
    id: QuestionId,
    title: String,
    conent: String,
    tags: Option<Vec<String>>,
}

#[derive(Clone)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, conent: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            conent,
            tags,
        }
    }

    fn update_title(&self, new_title: String) -> Self {
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

/// # `String` and `&str`
/// A quick summary:
/// * If you need own and modify the text,create a `String` type.
/// * Use `&str` when you need only a view of the underlying text.
/// * When creating new data type via a struct,you typically create `String` field types.
/// * When passing strings/text to a function,you usually use `&str`.

fn main() {
    let question = Question::new(
        QuestionId::from_str("1").unwrap(),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{}", question);

    let question = question.update_title("better_title".to_string());
    println!("{}", question);
}
