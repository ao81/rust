fn main() {
    let mut s = String::from("Hello World");

    let word = first_word(&mut s);
    println!("First word: {word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}