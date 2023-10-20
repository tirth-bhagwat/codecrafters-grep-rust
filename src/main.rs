mod tests;

use std::env;
use std::io;
use std::process;

#[derive(PartialEq)]
enum Mode {
    SameCharacter(char),
    Alphanumeric,
    PosCharGroup(Vec<char>),
    NegCharGroup(Vec<char>),
    Number,
    Any,
}
#[derive(PartialEq)]
enum MultipleMatch {
    ZeroOrOne { matched_empty: bool },
    OneOrMore,
    One,
}

#[derive(PartialEq)]
enum BreakLoop {
    Inner(bool),
    Outer,
}

fn match_pattern(pattern: &str, input_line: &str) -> Option<String> {
    let inp: Vec<char> = input_line.chars().collect();
    let mut patt: Vec<char> = pattern.chars().collect();
    let inp_len = inp.len();
    let patt_len = patt.len();

    let mut chars_matched = "".to_string();
    let mut reset_pattern = false;
    let mut starts_with = false;

    let mut match_only_next = false;

    let mut i = 0;
    let mut j = 0;
    while i < patt_len || reset_pattern {
        if reset_pattern {
            i = 0;
            reset_pattern = false;
            chars_matched = "".to_string();
            match_only_next = false;
        }

        let x = patt[i];
        i += 1;
        match x {
            '^' => {
                match_only_next = true;
                starts_with = true;
                continue;
            }
            '$' => {
                return if chars_matched.len() > 0 && j >= inp_len {
                    Some(chars_matched)
                } else if j < inp_len {
                    reset_pattern = true;
                    j -= 1;
                    continue;
                } else {
                    None
                }
            }
            _ => {}
        }

        let mut multi_match = None;
        let mut matched = 0_usize;
        'outer: while let Some(mode) = get_mode(&mut patt, x, &mut i) {
            if i < patt_len && multi_match.is_none() {
                multi_match = Some(match patt[i] {
                    '+' => {
                        i += 1;
                        MultipleMatch::OneOrMore
                    }
                    '?' => {
                        i += 1;
                        MultipleMatch::ZeroOrOne {
                            matched_empty: false,
                        }
                    }
                    _ => MultipleMatch::One,
                });
            } else if i == patt_len && multi_match.is_none() {
                multi_match = Some(MultipleMatch::One);
            }

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
                        Mode::Any => true,
                    };

                    if let Some(val) = get_break_condition(
                        &inp,
                        condition,
                        &mut match_only_next,
                        &mut chars_matched,
                        &mut multi_match,
                        &mut reset_pattern,
                        &mut j,
                        &mode,
                        starts_with,
                    ) {
                        match val {
                            BreakLoop::Inner(val) => {
                                break 'inner val;
                            }
                            BreakLoop::Outer => break 'outer,
                        }
                    }
                } else {
                    return None;
                }
            };

            if tmp {
                matched += 1;
            }

            match multi_match {
                Some(MultipleMatch::One) => {
                    if tmp {
                        break 'outer;
                    } else {
                        return None;
                    }
                }
                Some(MultipleMatch::OneOrMore) => {
                    if (matched > 0 && !tmp) || j == inp_len {
                        j -= 1;
                        break 'outer;
                    }
                }
                Some(MultipleMatch::ZeroOrOne { matched_empty }) => {
                    if (matched == 0 && !tmp) || matched == 1 || matched_empty {
                        if !tmp || matched_empty {
                            j -= 1;
                        }
                        break 'outer;
                    }
                }
                None => {
                    unreachable!()
                }
            }
        }
    }

    Some(chars_matched)
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

    if match_pattern(&pattern, &input_line).is_some() {
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
        '.' => Some(Mode::Any),
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
    input_line: &Vec<char>,
    condition: bool,
    match_only_next: &mut bool,
    chars_matched: &mut String,
    multi_match: &mut Option<MultipleMatch>,
    reset_pattern: &mut bool,
    j: &mut usize,
    mode: &Mode,
    starts_with: bool,
) -> Option<BreakLoop> {
    if *mode == Mode::Any {
        if let Some(MultipleMatch::ZeroOrOne { matched_empty: val }) = multi_match {
            if starts_with {
                return Some(BreakLoop::Inner(true));
            }
            return match val {
                false => {
                    *multi_match = Some(MultipleMatch::ZeroOrOne {
                        matched_empty: true,
                    });
                    (*chars_matched).push(input_line[*j - 1]);
                    Some(BreakLoop::Inner(true))
                }
                true => Some(BreakLoop::Inner(false)),
            };
        }
    }
    if condition {
        *match_only_next = true;
        (*chars_matched).push(input_line[*j - 1]);
        return Some(BreakLoop::Inner(true));
    } else if *match_only_next {
        if (*chars_matched).len() > 0 && *multi_match == Some(MultipleMatch::One) {
            *reset_pattern = true;
            *j -= 1;
            return Some(BreakLoop::Outer);
        }
        return Some(BreakLoop::Inner(false));
    }

    if let Some(MultipleMatch::ZeroOrOne { matched_empty: val }) = multi_match {
        return match val {
            false => {
                *multi_match = Some(MultipleMatch::ZeroOrOne {
                    matched_empty: true,
                });
                (*chars_matched).push(input_line[*j - 1]);
                Some(BreakLoop::Inner(true))
            }
            true => Some(BreakLoop::Inner(false)),
        };
    }

    return None;
}
