use std::env;
use std::io;
use std::process;

enum Mode {
    SameCharacter(char),
    Alphanumeric,
    Number,
    None,
    Unknown,
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut inp = input_line.chars();
    let mut patt = pattern.chars();
    let mut mode = Mode::Unknown;

    while let Some(x) = patt.next() {
        if x.is_ascii_alphabetic() {
            mode = Mode::SameCharacter(x);
        } else if x == '\\' {
            if let Some(u) = patt.next() {
                match u {
                    'w' => mode = Mode::Alphanumeric,
                    'd' => mode = Mode::Number,
                    _ => {
                        return false;
                    }
                }
            }
        }

        match mode {
            Mode::SameCharacter(ch) => {
                if let Some(x) = inp.next() {
                    if !x.is_ascii_alphabetic() || x != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            Mode::Alphanumeric => {
                if let Some(x) = inp.next() {
                    if !(x.is_ascii_alphanumeric() || x == '_') {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            Mode::Number => {
                if let Some(x) = inp.next() {
                    if !x.is_ascii_digit() {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().len() < 3 {
        process::exit(1);
    }

    if env::args().nth(1).unwrap() != "-E" {
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        println!("Pass");
        process::exit(0)
    } else {
        println!("Fail");
        process::exit(1)
    }
}
