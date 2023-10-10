use std::env;
use std::io;
use std::process;

enum Mode {
    SameCharacter(char),
    Alphanumeric,
    PosCharGroup(Vec<char>),
    NegCharGroup(Vec<char>),
    Number,
    Unknown,
}

fn match_pattern(pattern: &str, input_line: &str) -> bool {
    let mut inp = input_line.chars();
    let mut patt = pattern.chars();
    let mut mode = Mode::Unknown;

    let mut match_only_next = false;

    while let Some(x) = patt.next() {
        match x {
            '\\' => {
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
            '[' => {
                let mut accepted: Vec<char> = vec![];
                let mut pos = true;

                if let Some(ch) = patt.next() {
                    if ch == '^' {
                        pos = false;
                    } else {
                        accepted.push(ch);
                    }
                }

                accepted.append(
                    (&mut patt)
                        .take_while(|ch| *ch != ']')
                        .collect::<Vec<char>>()
                        .as_mut(),
                );
                if pos {
                    mode = Mode::PosCharGroup(accepted);
                } else {
                    mode = Mode::NegCharGroup(accepted);
                }
            }

            _ => {
                mode = Mode::SameCharacter(x);
            }
        }

        loop {
            if let Some(y) = inp.next() {
                match &mode {
                    Mode::SameCharacter(ch) => {
                        if &y == ch {
                            match_only_next = true;
                            break;
                        } else if match_only_next {
                            return false;
                        }
                    }
                    Mode::Alphanumeric => {
                        if y.is_ascii_alphanumeric() || y == '_' {
                            match_only_next = true;
                            break;
                        } else if match_only_next {
                            return false;
                        }
                    }
                    Mode::PosCharGroup(accepted) => {
                        if accepted.contains(&y) {
                            match_only_next = true;
                            break;
                        } else if match_only_next {
                            return false;
                        }
                    }
                    Mode::NegCharGroup(not_accepted) => {
                        if !not_accepted.contains(&y) {
                            match_only_next = true;
                            break;
                        } else if match_only_next {
                            return false;
                        }
                    }
                    Mode::Number => {
                        if y.is_ascii_digit() {
                            match_only_next = true;
                            break;
                        } else if match_only_next {
                            return false;
                        }
                    }
                    Mode::Unknown => return false,
                }
            } else {
                return false;
            }
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

    if match_pattern(&pattern, &input_line) {
        println!("Pass");
        process::exit(0)
    } else {
        println!("Fail");
        process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_1() {
        assert_eq!(match_pattern("abc", "abc"), true, "Test 1");
        assert_eq!(match_pattern("abc", "aibic"), false, "Test 2");
        assert_eq!(match_pattern("abc", "aaibicj"), false, "Test 3");
        assert_eq!(match_pattern("abc", "abcdef"), true, "Test 4");
        assert_eq!(match_pattern("abc", "defabc"), true, "Test 5");
        assert_eq!(match_pattern("abc", "defabcdef"), true, "Test 6");
    }
    #[test]
    fn test_stage_2() {
        assert_eq!(match_pattern("\\d", "12345"), true, "Test 1");
        assert_eq!(match_pattern("\\d", "abc"), false, "Test 2");
        assert_eq!(match_pattern("\\d", "apple123"), true, "Test 3");
        assert_eq!(match_pattern("\\d", "123apple"), true, "Test 4");
        assert_eq!(match_pattern("\\d", "apple"), false, "Test 5");
        assert_eq!(match_pattern("\\d", "123"), true, "Test 6");
        assert_eq!(match_pattern("\\d", "a1b2c3"), true, "Test 7");
        assert_eq!(match_pattern("\\d", "a1b2c"), true, "Test 8");
    }

    #[test]
    fn test_stage_3() {
        assert_eq!(match_pattern("\\w", "foo101"), true, "Test 1");
        assert_eq!(match_pattern("\\w", "$!?"), false, "Test 2");
        assert_eq!(match_pattern("\\w", "alpha-num3ric"), true, "Test 3");
        assert_eq!(match_pattern("\\w", "_underscore"), true, "Test 4");
        assert_eq!(match_pattern("\\w", "%:'"), false, "Test 5");
        assert_eq!(match_pattern("\\w", "12345"), true, "Test 6");
        assert_eq!(match_pattern("\\w", "AbC"), true, "Test 7");
        assert_eq!(match_pattern("\\w", "_"), true, "Test 8");
        assert_eq!(match_pattern("\\w", "a"), true, "Test 9");
        assert_eq!(match_pattern("\\w", "9"), true, "Test 10");
        assert_eq!(match_pattern("\\w", "!"), false, "Test 11");
    }
    #[test]
    fn test_stage_4() {
        assert_eq!(match_pattern("[abc]", "apple"), true, "Test 1");
        assert_eq!(match_pattern("[123]", "apple"), false, "Test 2");
        assert_eq!(match_pattern("[xyz]", "XYZ"), false, "Test 3");
        assert_eq!(match_pattern("[aeiou]", "consonants"), true, "Test 4");
        assert_eq!(match_pattern("[aeiou]", "rhythm"), false, "Test 5");
        // assert_eq!(match_pattern("[a-z]", "abcdefg"), true, "Extra_1");
        // assert_eq!(match_pattern("[0-9]", "alpha123"), true, "Extra_2");
    }

    #[test]
    fn test_stage_5() {
        assert_eq!(match_pattern("[^abc]", "dog"), true, "Test 1");
        assert_eq!(match_pattern("[^abc]", "cab"), false, "Test 2");
        assert_eq!(match_pattern("[^pqr]", "apple"), true, "Test 3");
        assert_eq!(
            match_pattern("[^aeiou][^aeiou]", "consonants"),
            false,
            "Test 4"
        );
        assert_eq!(match_pattern("[^aeiou]", "rhythm"), true, "Test 5");
        assert_eq!(match_pattern("[^123]", "456"), true, "Test 6");
        // assert_eq!(match_pattern("[^a-z]", "123"), true, "Extra_1");
        // assert_eq!(match_pattern("[^0-9]", "alpha"), true, "Extra_2");
    }

    #[test]
    fn test_stage_6() {
        assert_eq!(match_pattern("\\d apple", "1 apple"), true, "Test 1");
        assert_eq!(match_pattern("\\d apple", "1 orange"), false, "Test 2");
        assert_eq!(
            match_pattern("\\d\\d\\d apple", "100 apples"),
            true,
            "Test 3"
        );
        assert_eq!(match_pattern("\\d\\d\\d apple", "1 apple"), false, "Test 4");
        assert_eq!(match_pattern("\\d \\w\\w\\ws", "3 dogs"), true, "Test 5");
        assert_eq!(match_pattern("\\d \\w\\w\\ws", "4 cats"), true, "Test 6");
        assert_eq!(match_pattern("\\d \\w\\w\\ws", "1 dog"), false, "Test 7");
        assert_eq!(
            match_pattern("\\d\\w\\w\\w apple", "1dog apple"),
            true,
            "Test 8"
        );
    }
}
