#[cfg(test)]
mod tests {
    use crate::match_pattern;

    fn run_tests(lst: Vec<(&str, &str, Option<String>, &str)>) {
        for (pattern, input_line, expected, test_name) in lst {
            assert_eq!(
                match_pattern(pattern, input_line),
                expected,
                "Test {}",
                test_name
            );
        }
    }
    #[test]
    fn test_stage_1() {
        let lst = vec![
            ("abc", "abc", Some("abc".to_string()), "1"),
            ("abc", "aibic", None, "2"),
            ("abc", "aaibicj", None, "3"),
            ("abc", "aabcibicj", Some("abc".to_string()), "3b"),
            ("abc", "abcdef", Some("abc".to_string()), "4"),
            ("abc", "defabc", Some("abc".to_string()), "5"),
            ("abc", "defabcdef", Some("abc".to_string()), "6"),
        ];

        run_tests(lst);
    }
    #[test]
    fn test_stage_2() {
        // Stage 2
        let lst = vec![
            ("\\d", "12345", Some("1".to_string()), "1"),
            ("\\d", "abc", None, "2"),
            ("\\d", "apple123", Some("1".to_string()), "3"),
            ("\\d", "123apple", Some("1".to_string()), "4"),
            ("\\d", "apple", None, "5"),
            ("\\d", "123", Some("1".to_string()), "6"),
            ("\\d", "a1b2c3", Some("1".to_string()), "7"),
            ("\\d", "a1b2c", Some("1".to_string()), "8"),
        ];
        run_tests(lst);
    }
    #[test]
    fn test_stage_3() {
        // Stage 3
        let lst = vec![
            ("\\w", "foo101", Some("f".to_string()), "1"),
            ("\\w", "$!?", None, "2"),
            ("\\w", "alpha-num3ric", Some("a".to_string()), "3"),
            ("\\w", "_underscore", Some("_".to_string()), "4"),
            ("\\w", "%:'", None, "5"),
            ("\\w", "12345", Some("1".to_string()), "6"),
            ("\\w", "AbC", Some("A".to_string()), "7"),
            ("\\w", "_", Some("_".to_string()), "8"),
            ("\\w", "a", Some("a".to_string()), "9"),
            ("\\w", "9", Some("9".to_string()), "10"),
            ("\\w", "!", None, "11"),
        ];
        run_tests(lst);
    }
    #[test]
    fn test_stage_4() {
        // Stage 4
        let lst = vec![
            ("[abc]", "apple", Some("a".to_string()), "1"),
            ("[123]", "apple", None, "2"),
            ("[xyz]", "XYZ", None, "3"),
            ("[aeiou]", "consonants", Some("o".to_string()), "4"),
            ("[aeiou]", "rhythm", None, "5"),
            ("[aeiou][aeiou]", "consonants", None, "6"),
        ];
        run_tests(lst);
    }
    #[test]
    fn test_stage_5() {
        // Stage 5
        let lst = vec![
            ("[^abc]", "dog", Some("d".to_string()), "1"),
            ("[^abc]", "cab", None, "2"),
            ("[^pqr]", "apple", Some("a".to_string()), "3"),
            (
                "[^aeiou][^aeiou]",
                "consonants",
                Some("ns".to_string()),
                "4",
            ),
            ("[^aeiou]", "rhythm", Some("r".to_string()), "5"),
            ("[^123]", "456", Some("4".to_string()), "6"),
        ];
        run_tests(lst);
    }
    #[test]
    fn test_stage_6() {
        // Stage 6
        let lst = vec![
            ("\\d apple", "1 apple", Some("1 apple".to_string()), "1"),
            ("\\d apple", "1 orange", None, "2"),
            (
                "\\d\\d\\d apple",
                "100 apples",
                Some("100 apple".to_string()),
                "3",
            ),
            ("\\d\\d\\d apple", "1 apple", None, "4"),
            ("\\d \\w\\w\\ws", "3 dogs", Some("3 dogs".to_string()), "5"),
            ("\\d \\w\\w\\ws", "4 cats", Some("4 cats".to_string()), "6"),
            ("\\d \\w\\w\\ws", "1 dog", None, "7"),
            (
                "\\d\\w\\w\\w apple",
                "1dog apple",
                Some("1dog apple".to_string()),
                "8",
            ),
        ];
        run_tests(lst);
    }
    #[test]
    fn test_stage_7() {
        // Stage 7
        let lst = vec![
            ("^log", "log", Some("log".to_string()), "1"),
            ("^log", "slog", None, "2"),
            ("^apple", "apple pie", Some("apple".to_string()), "3"),
            ("^apple", "pie apple", None, "4"),
            ("^123", "123456", Some("123".to_string()), "5"),
            ("^123", "456123", None, "6"),
        ];
        run_tests(lst);
    }
    #[test]
    fn test_stage_8() {
        // Stage 8
        let lst = vec![
            ("dog$", "dog", Some("dog".to_string()), "1"),
            ("^dog$", "dog", Some("dog".to_string()), "1a"),
            ("dog$", "dogs", None, "2"),
            ("pie$", "apple pie", Some("pie".to_string()), "3"),
            ("^pie$", "apple pie", None, "3a"),
            ("apple$", "pie apple", Some("apple".to_string()), "4"),
            ("apple$", "pie apple appl", None, "4a"),
            ("apple$", "pie apple apple", Some("apple".to_string()), "4b"),
            ("123$", "123456", None, "5"),
            ("123$", "456123", Some("123".to_string()), "6"),
            ("efgh$", "abcd\nefgh", Some("efgh".to_string()), "7"),
            ("abcd$", "abcd\nefgh", None, "8"),
        ];
        run_tests(lst);
    }
    #[test]
    fn test_stage_9() {
        // Stage 9
        let lst = vec![
            ("a+", "apple", Some("a".to_string()), "1"),
            ("a+", "SaaS", Some("aa".to_string()), "2"),
            ("a+", "dog", None, "3"),
            ("ca+ts", "cats", Some("cats".to_string()), "4"),
            ("ca+ts", "caats", Some("caats".to_string()), "5"),
            ("ca+ts", "caaaats", Some("caaaats".to_string()), "6"),
            ("ca+ts", "ctss", None, "7"),
            ("ca+ts", "cass caats", Some("caats".to_string()), "8"),
            ("^ca+ts", "cass caats", None, "9"),
            ("ca+ts$", "cass caats", Some("caats".to_string()), "9a"),
            ("ca+ts$", "cats cass", None, "9b"),
        ];
        run_tests(lst);
    }
    #[test]
    fn test_stage_10() {
        // Stage 10
        let lst = vec![
            ("a?", "apple", Some("a".to_string()), "1"),
            ("a?", "SaaS", Some("S".to_string()), "2"),
            ("ca?ts", "cats", Some("cats".to_string()), "4"),
            ("ca?ts", "caats", None, "5"),
            ("ca?ts", "caaaats", None, "6"),
            ("ca?ts", "cass cats", Some("cats".to_string()), "8"),
            ("ca?ts", "cass caats", None, "9"),
            ("^ca?ts", "cass cats", None, "10"),
            ("a?", "dog", Some("d".to_string()), "3"),
            ("ca?ts", "cts", Some("cts".to_string()), "7"),
            ("ca?ts", "cass cts", Some("cts".to_string()), "8a"),
        ];
        run_tests(lst);
    }
    #[test]
    fn test_stage_11() {
        // Stage 11
        let lst = vec![
            ("d.g", "dog", Some("dog".to_string()), "1"),
            ("d.g", "dug", Some("dug".to_string()), "2"),
            ("d.g", "d#g", Some("d#g".to_string()), "3"),
            ("d.g", "dig", Some("dig".to_string()), "4"),
            (".at", "cat", Some("cat".to_string()), "5"),
            (".at", "bat", Some("bat".to_string()), "6"),
            (".at", "rat", Some("rat".to_string()), "7"),
            ("b.t", "bat", Some("bat".to_string()), "8"),
            ("b.t", "btt", Some("btt".to_string()), "9"),
            ("b.t", "boot", None, "10"),
            ("...", "abc", Some("abc".to_string()), "11"),
            ("...", "!@#", Some("!@#".to_string()), "13"),
            (".+", "apple", Some("apple".to_string()), "15"),
            (".+", "123", Some("123".to_string()), "16"),
            (".+", "!@#", Some("!@#".to_string()), "17"),
            ("x.y", "xyz", None, "19"),
            ("x.y", "xty", Some("xty".to_string()), "20"),
            ("x.y", "x1y", Some("x1y".to_string()), "21"),
            ("x.y", "xy", None, "22"),
            (".?at", "cat", Some("cat".to_string()), "23"),
            (".?at", "bat", Some("bat".to_string()), "24"),
            (".?at", "rrrat", Some("rat".to_string()), "25"),
            ("^.?at", "rrrat", None, "25b"),
            (".?at$", "rrrat", Some("rat".to_string()), "25c"),
            (".?at$", "ratat", Some("tat".to_string()), "25d"),
            (".?at$", "rataq", None, "25e"),
            (".?at", "at", Some("at".to_string()), "26"),
            (".?at", "hhh", None, "27"),
            ("a.b.c.", "adbecf", Some("adbecf".to_string()), "28"),
        ];
        run_tests(lst);
    }

    // #[test]
    // fn test_stage_12() {
    //     assert_eq!(match_pattern("(cat|dog)", "dog"), true, "Test 1");
    //     assert_eq!(match_pattern("(cat|dog)", "cat"), true, "Test 2");
    //     assert_eq!(match_pattern("(cat|dog)", "apple"), false, "Test 3");
    //     assert_eq!(match_pattern("(apple|banana)", "banana"), true, "Test 4");
    //     assert_eq!(match_pattern("(apple|banana)", "apple"), true, "Test 5");
    //     assert_eq!(match_pattern("(apple|banana)", "cherry"), false, "Test 6");
    //     assert_eq!(match_pattern("(123|456)", "123"), true, "Test 7");
    //     assert_eq!(match_pattern("(123|456)", "456"), true, "Test 8");
    //     assert_eq!(match_pattern("(123|456)", "789"), false, "Test 9");
    //     assert_eq!(match_pattern("(abc|def)", "abc"), true, "Test 10");
    //     assert_eq!(match_pattern("(abc|def)", "def"), true, "Test 11");
    //     assert_eq!(match_pattern("(abc|def)", "ghi"), false, "Test 12");
    //     assert_eq!(
    //         match_pattern("(cat|dog) (and|or) (fish|bird)", "cat and fish"),
    //         true,
    //         "Test 13"
    //     );
    //     assert_eq!(
    //         match_pattern("(cat|dog) (and|or) (fish|bird)", "dog or bird"),
    //         true,
    //         "Test 14"
    //     );
    //     assert_eq!(
    //         match_pattern("(cat|dog) (and|or) (fish|bird)", "cat and bird"),
    //         false,
    //         "Test 15"
    //     );
    // }
}
// assert_eq!(match_pattern("[^a-z]", "123"), true, "Extra_1");
// assert_eq!(match_pattern("[^0-9]", "alpha"), true, "Extra_2");

// assert_eq!(match_pattern("^ab", "abcd\nefgh"), true, "Test 7");
// assert_eq!(match_pattern("^cd", "abcd\nefgh"), false, "Test 8");
