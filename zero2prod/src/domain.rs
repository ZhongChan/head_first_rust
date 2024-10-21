use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscirberName,
}

pub struct SubscirberName(String);

impl SubscirberName {
    pub fn inner_mut(&mut self) -> &mut str {
        &mut self.0
    }

    pub fn parse(s: String) -> Result<SubscirberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters) {
            panic!("{} is not a valid subscriber name.", s)
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscirberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
