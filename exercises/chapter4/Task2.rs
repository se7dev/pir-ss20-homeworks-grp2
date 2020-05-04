fn main() {
    let s = String::from("Hello World");
    let word = first_word(&s[..]);
    let word2 = second_word(&s[..]);
    println!("First word {}", word);
    println!("Second word {}", word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let mut second_index = 0;
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if second_index == 0 {
                second_index = i + 1;
            } else { return &s[second_index..i + 1]; }
        }
        if i == s.len() - 1 {
            if second_index > 0 {
                return &s[second_index..i + 1];
            } else {
                return &s[0..i + 1];
            }
        }
    }
    &s[..]
}