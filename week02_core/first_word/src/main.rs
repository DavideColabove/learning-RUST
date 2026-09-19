
fn main() {
    let s = String::from("hello World!");

    println!("{}, {}", first_word(&s), second_word(&s))
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start == 0 {
                start = i + 1;
            } else {
                return &s[start..i];
            }
        }
    }

    if start > 0 {
        &s[start..]
    } else {
        ""
    }
}