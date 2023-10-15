use std::env;
use std::io;
use std::process;

enum Mode {
    SameCharacter(char),
    Alphanumeric,
    PosCharGroup(Vec<char>),
    NegCharGroup(Vec<char>),
    Number,
}
#[derive(PartialEq)]
enum MultipleMatch {
    ZeroOrOne { matched_empty: bool },
    OneOrMore,
    One,
}

enum BreakWhat {
    Inner(bool),
    Outer,
}

fn match_pattern(pattern: &str, input_line: &str) -> bool {
    let inp: Vec<char> = input_line.chars().collect();
    let mut patt: Vec<char> = pattern.chars().collect();
    let inp_len = inp.len();
    let patt_len = patt.len();

    let mut chars_matched = 0_usize;
    let mut reset_pattern = false;

    let mut match_only_next = false;

    let mut i = 0;
    let mut j = 0;
    while i < patt_len || reset_pattern {
        if reset_pattern {
            i = 0;
            reset_pattern = false;
            chars_matched = 0;
            match_only_next = false;
        }

        let x = patt[i];
        i += 1;
        match x {
            '^' => {
                match_only_next = true;
                continue;
            }
            '$' => {
                return if chars_matched > 0 && j >= inp_len {
                    true
                } else if j < inp_len {
                    reset_pattern = true;
                    continue;
                } else {
                    false
                }
            }
            _ => {}
        }

        let mut multi_match = MultipleMatch::One;
        if i < patt_len {
            multi_match = match patt[i] {
                '+' => {
                    i += 1;
                    MultipleMatch::OneOrMore
                }
                '?' => {
                    i += 1;
                    // match_only_next = true;
                    MultipleMatch::ZeroOrOne {
                        matched_empty: false,
                    }
                }
                _ => MultipleMatch::One,
            };
        }

        let mut matched = 0_usize;
        'outer: while let Some(mode) = get_mode(&mut patt, x, &mut i) {
            let tmp = 'inner: loop {
                if j < inp_len {
                    let y = inp[j];
                    j += 1;

                    let condition = match &mode {
                        Mode::SameCharacter(ch) => &y == ch,
                        Mode::Alphanumeric => y.is_ascii_alphanumeric() || y == '_',
                        Mode::PosCharGroup(accepted) => accepted.contains(&y),
                        Mode::NegCharGroup(not_accepted) => !not_accepted.contains(&y),
                        Mode::Number => y.is_ascii_digit(),
                    };

                    if let Some(val) = get_break_condition(
                        condition,
                        &mut match_only_next,
                        &mut chars_matched,
                        &mut multi_match,
                        &mut reset_pattern,
                        &mut j,
                    ) {
                        match val {
                            BreakWhat::Inner(val) => {
                                break 'inner val;
                            }
                            BreakWhat::Outer => break 'outer,
                        }
                    }
                } else {
                    return false;
                }
            };

            if tmp {
                matched += 1;
            }

            match multi_match {
                MultipleMatch::One => {
                    if tmp {
                        break 'outer;
                    } else {
                        return false;
                    }
                }
                MultipleMatch::OneOrMore => {
                    if matched > 0 && !tmp {
                        j -= 1;
                        break 'outer;
                    }
                }
                MultipleMatch::ZeroOrOne { matched_empty: _ } => {
                    if (matched == 0 || matched == 1) && !tmp {
                        j -= 1;
                        break 'outer;
                    }
                }
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

fn get_mode(patt: &mut Vec<char>, x: char, i: &mut usize) -> Option<Mode> {
    let patt_len = patt.len();
    return match x {
        '\\' => {
            if *i < patt_len {
                let u = patt[*i];
                *i += 1;
                return match u {
                    'w' => Some(Mode::Alphanumeric),
                    'd' => Some(Mode::Number),
                    _ => None,
                };
            }
            None
        }
        '[' => {
            let mut accepted: Vec<char> = vec![];
            let mut pos = true;

            if *i < patt_len {
                let ch = patt[*i];
                *i += 1;
                if ch == '^' {
                    pos = false;
                } else {
                    accepted.push(ch);
                }
            }

            accepted.append(
                patt.clone()[*i..]
                    .iter()
                    .take_while(|ch| **ch != ']')
                    .map(|x| *x)
                    .collect::<Vec<char>>()
                    .as_mut(),
            );
            if pos {
                *i += accepted.len();
                Some(Mode::PosCharGroup(accepted))
            } else {
                *i += accepted.len() + 1;
                Some(Mode::NegCharGroup(accepted))
            }
        }
        _ => Some(Mode::SameCharacter(x)),
    };
}

fn get_break_condition(
    condition: bool,
    match_only_next: &mut bool,
    chars_matched: &mut usize,
    multi_match: &mut MultipleMatch,
    reset_pattern: &mut bool,
    j: &mut usize,
) -> Option<BreakWhat> {
    if condition {
        *match_only_next = true;
        *chars_matched += 1;
        return Some(BreakWhat::Inner(true));
    } else if *match_only_next {
        if *chars_matched > 0 && *multi_match == MultipleMatch::One {
            *reset_pattern = true;
            *j -= 1;
            return Some(BreakWhat::Outer);
        }
        return Some(BreakWhat::Inner(false));
    } else if let MultipleMatch::ZeroOrOne { matched_empty: val } = multi_match {
        return match val {
            false => {
                *multi_match = MultipleMatch::ZeroOrOne {
                    matched_empty: true,
                };
                Some(BreakWhat::Inner(true))
            }
            true => Some(BreakWhat::Inner(false)),
        };
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_1() {
        assert_eq!(match_pattern("abc", "abc"), true, "Test 1");
        assert_eq!(match_pattern("abc", "aibic"), false, "Test 2");
        assert_eq!(match_pattern("abc", "aaibicj"), false, "Test 3");
        assert_eq!(match_pattern("abc", "aabcibicj"), true, "Test 3b");
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
        assert_eq!(
            match_pattern("[aeiou][aeiou]", "consonants"),
            false,
            "Test 6"
        );
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
            true,
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
    #[test]
    fn test_stage_7() {
        assert_eq!(match_pattern("^log", "log"), true, "Test 1");
        assert_eq!(match_pattern("^log", "slog"), false, "Test 2");
        assert_eq!(match_pattern("^apple", "apple pie"), true, "Test 3");
        assert_eq!(match_pattern("^apple", "pie apple"), false, "Test 4");
        assert_eq!(match_pattern("^123", "123456"), true, "Test 5");
        assert_eq!(match_pattern("^123", "456123"), false, "Test 6");
        // assert_eq!(match_pattern("^ab", "abcd\nefgh"), true, "Test 7");
        // assert_eq!(match_pattern("^cd", "abcd\nefgh"), false, "Test 8");
    }
    #[test]
    fn test_stage_8() {
        assert_eq!(match_pattern("dog$", "dog"), true, "Test 1");
        assert_eq!(match_pattern("^dog$", "dog"), true, "Test 1a");
        assert_eq!(match_pattern("dog$", "dogs"), false, "Test 2");
        assert_eq!(match_pattern("pie$", "apple pie"), true, "Test 3");
        assert_eq!(match_pattern("^pie$", "apple pie"), false, "Test 3a");
        assert_eq!(match_pattern("apple$", "pie apple"), true, "Test 4");
        assert_eq!(match_pattern("apple$", "pie apple appl"), false, "Test 4a");
        assert_eq!(match_pattern("apple$", "pie apple apple"), true, "Test 4b");
        assert_eq!(match_pattern("123$", "123456"), false, "Test 5");
        assert_eq!(match_pattern("123$", "456123"), true, "Test 6");
        assert_eq!(match_pattern("efgh$", "abcd\nefgh"), true, "Test 7");
        assert_eq!(match_pattern("abcd$", "abcd\nefgh"), false, "Test 8");
    }
    #[test]
    fn test_stage_9() {
        assert_eq!(match_pattern("a+", "apple"), true, "Test 1");
        assert_eq!(match_pattern("a+", "SaaS"), true, "Test 2");
        assert_eq!(match_pattern("a+", "dog"), false, "Test 3");
        assert_eq!(match_pattern("ca+ts", "cats"), true, "Test 4");
        assert_eq!(match_pattern("ca+ts", "caats"), true, "Test 5");
        assert_eq!(match_pattern("ca+ts", "caaaats"), true, "Test 6");
        assert_eq!(match_pattern("ca+ts", "ctss"), false, "Test 7");
        assert_eq!(match_pattern("ca+ts", "cass caats"), true, "Test 8");
        assert_eq!(match_pattern("^ca+ts", "cass caats"), false, "Test 9");
    }
    #[test]
    fn test_stage_10() {
        assert_eq!(match_pattern("a?", "apple"), true, "Test 1");
        assert_eq!(match_pattern("a?", "SaaS"), false, "Test 2");
        assert_eq!(match_pattern("ca?ts", "cats"), true, "Test 4");
        assert_eq!(match_pattern("ca?ts", "caats"), false, "Test 5");
        assert_eq!(match_pattern("ca?ts", "caaaats"), false, "Test 6");
        assert_eq!(match_pattern("ca?ts", "cass cats"), true, "Test 8");
        assert_eq!(match_pattern("ca?ts", "cass caats"), false, "Test 9");
        assert_eq!(match_pattern("^ca?ts", "cass cats"), false, "Test 10");

        assert_eq!(match_pattern("a?", "dog"), true, "Test 3");
        assert_eq!(match_pattern("ca?ts", "cts"), true, "Test 7");
        assert_eq!(match_pattern("ca?ts", "cass cts"), true, "Test 8a");
    }
}
