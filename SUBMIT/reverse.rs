fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = "hello";
    println!("Reversed string: {}", reverse_string(s));
}
