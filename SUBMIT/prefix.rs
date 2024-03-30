fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = String::new();
    for (i, &byte) in strs[0].as_bytes().iter().enumerate() {
        if strs.iter().any(|&s| s.as_bytes().get(i) != Some(&byte)) {
            break;
        }
        prefix.push(byte as char);
    }
    prefix
}

fn main() {
    let strs = vec!["flower", "flow", "flight"];
    println!("Longest common prefix: {}", longest_common_prefix(&strs));
}
