pub fn reverse(input: &str) -> String {
    let mut input_chars: Vec<char> = input.chars().collect();
    input_chars.reverse();
    input_chars.into_iter().collect()
}
