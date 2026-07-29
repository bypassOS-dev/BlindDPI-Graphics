use std::collections::HashMap;

fn main() {
    let text: &str = "Rust - is the beast programming language. Who doesn'n love Rust? Everyone loves Rust!";

    let content: Vec<String> = text.split_whitespace().map(|word| word.to_lowercase()).collect();

    let mut how_much: HashMap< String, i32 > = HashMap::new();

}