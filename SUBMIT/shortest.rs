fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|&word| word.len())
}

fn main() {
    let input = "This is a test string";
    if let Some(shortest) = shortest_word(input) {
        println!("Shortest word: {}", shortest);
    } else {
        println!("No words in the string");
    }
}
