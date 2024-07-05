#[derive(Debug, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    conent: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
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

/// # `String` and `&str`
/// A quick summary:
/// * If you need own and modify the text,create a `String` type.
/// * Use `&str` when you need only a view of the underlying text.
/// * When creating new data type via a struct,you typically create `String` field types.
/// * When passing strings/text to a function,you usually use `&str`.

fn main() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:?}", question);

    let question = question.update_title("better_title".to_string());
    println!("{:?}", question);
}
