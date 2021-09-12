use std::io::{Read, Result, stdin};

#[derive(Debug)]
struct Token<'a> {
    s: &'a str,
    start_index: usize,
}

fn tokenize<'a>(code: &'a str) -> impl Iterator<Item=Token<'a>> {
    let mut next_token = 0;
    let code_length = code.len();
    code.char_indices()
        .filter_map(move |(index, c)|
            if c.is_whitespace() {
                let start_index = next_token;
                next_token = index + 1;
                Some(Token { s: &code[start_index..index], start_index })
            } else if index == code_length - 1 {
                Some(Token { s: &code[next_token..index + 1], start_index: next_token })
            } else {
                None
            })
}

fn main() -> Result<()> {
    let mut buffer = "Holaaa@ bebe!!".to_string();

    for token in tokenize(buffer.as_str()) {
        print!("{:?} ", token);
    }
    Ok(())
}
