#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ch_02::Question;
    use ch_02::QuestionId;

    #[test]
    fn test_question_creation() {
        let question = Question::new(
            QuestionId::from_str("1").unwrap(),
            "First Question".to_string(),
            "Conent of question".to_string(),
            Some(vec!["faq".to_string(), "web".to_string()]),
        );

        assert_eq!(question.title, "First Question");
        assert_eq!(question.conent, "Conent of question");
        assert_eq!(
            question.tags,
            Some(vec!["faq".to_string(), "web".to_string()])
        );
        println!("{}", question);
    }

    #[test]
    fn test_update_title() {
        let question = Question::new(
            QuestionId::from_str("1").unwrap(),
            "First Question".to_string(),
            "Conent of question".to_string(),
            Some(vec!["faq".to_string(), "web".to_string()]),
        );

        let update_question = question.update_title("better title".to_string());
        assert_eq!(update_question.title, "better title");
        println!("{}", update_question);
    }

    #[test]
    #[should_panic = "value borrowed here after move"]
    fn test_ownership() {
        let x = String::from("Hello.");
        let y = x;

        // Uncommenting the next line will cause a compile error
        // println!("{}", x); // error: borrow of moved value: `x`
    }
}
