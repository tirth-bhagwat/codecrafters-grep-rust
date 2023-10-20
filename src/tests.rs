
#[cfg(test)]
mod tests {
    use crate::match_pattern;

    #[test]
    fn test_stage_1() {
        assert_eq!(
            match_pattern("abc", "abc"),
            Some("abc".to_string()),
            "Test 1"
        );
        assert_eq!(match_pattern("abc", "aibic"), None, "Test 2");
        assert_eq!(match_pattern("abc", "aaibicj"), None, "Test 3");
        assert_eq!(
            match_pattern("abc", "aabcibicj"),
            Some("abc".to_string()),
            "Test 3b"
        );
        assert_eq!(
            match_pattern("abc", "abcdef"),
            Some("abc".to_string()),
            "Test 4"
        );
        assert_eq!(
            match_pattern("abc", "defabc"),
            Some("abc".to_string()),
            "Test 5"
        );
        assert_eq!(
            match_pattern("abc", "defabcdef"),
            Some("abc".to_string()),
            "Test 6"
        );
    }
    #[test]
    fn test_stage_2() {
        assert_eq!(
            match_pattern("\\d", "12345"),
            Some("1".to_string()),
            "Test 1"
        );
        assert_eq!(match_pattern("\\d", "abc"), None, "Test 2");
        assert_eq!(
            match_pattern("\\d", "apple123"),
            Some("1".to_string()),
            "Test 3"
        );
        assert_eq!(
            match_pattern("\\d", "123apple"),
            Some("1".to_string()),
            "Test 4"
        );
        assert_eq!(match_pattern("\\d", "apple"), None, "Test 5");
        assert_eq!(match_pattern("\\d", "123"), Some("1".to_string()), "Test 6");
        assert_eq!(
            match_pattern("\\d", "a1b2c3"),
            Some("1".to_string()),
            "Test 7"
        );
        assert_eq!(
            match_pattern("\\d", "a1b2c"),
            Some("1".to_string()),
            "Test 8"
        );
    }

    #[test]
    fn test_stage_3() {
        assert_eq!(
            match_pattern("\\w", "foo101"),
            Some("f".to_string()),
            "Test 1"
        );
        assert_eq!(match_pattern("\\w", "$!?"), None, "Test 2");
        assert_eq!(
            match_pattern("\\w", "alpha-num3ric"),
            Some("a".to_string()),
            "Test 3"
        );
        assert_eq!(
            match_pattern("\\w", "_underscore"),
            Some("_".to_string()),
            "Test 4"
        );
        assert_eq!(match_pattern("\\w", "%:'"), None, "Test 5");
        assert_eq!(
            match_pattern("\\w", "12345"),
            Some("1".to_string()),
            "Test 6"
        );
        assert_eq!(match_pattern("\\w", "AbC"), Some("A".to_string()), "Test 7");
        assert_eq!(match_pattern("\\w", "_"), Some("_".to_string()), "Test 8");
        assert_eq!(match_pattern("\\w", "a"), Some("a".to_string()), "Test 9");
        assert_eq!(match_pattern("\\w", "9"), Some("9".to_string()), "Test 10");
        assert_eq!(match_pattern("\\w", "!"), None, "Test 11");
    }

    #[test]
    fn test_stage_4() {
        assert_eq!(
            match_pattern("[abc]", "apple"),
            Some("a".to_string()),
            "Test 1"
        );
        assert_eq!(match_pattern("[123]", "apple"), None, "Test 2");
        assert_eq!(match_pattern("[xyz]", "XYZ"), None, "Test 3");
        assert_eq!(
            match_pattern("[aeiou]", "consonants"),
            Some("o".to_string()),
            "Test 4"
        );
        assert_eq!(match_pattern("[aeiou]", "rhythm"), None, "Test 5");
        assert_eq!(
            match_pattern("[aeiou][aeiou]", "consonants"),
            None,
            "Test 6"
        );
        // Add more tests if needed.
    }

    #[test]
    fn test_stage_5() {
        assert_eq!(
            match_pattern("[^abc]", "dog"),
            Some("d".to_string()),
            "Test 1"
        );
        assert_eq!(match_pattern("[^abc]", "cab"), None, "Test 2");
        assert_eq!(
            match_pattern("[^pqr]", "apple"),
            Some("a".to_string()),
            "Test 3"
        );
        assert_eq!(
            match_pattern("[^aeiou][^aeiou]", "consonants"),
            Some("ns".to_string()),
            "Test 4"
        );
        assert_eq!(
            match_pattern("[^aeiou]", "rhythm"),
            Some("r".to_string()),
            "Test 5"
        );
        assert_eq!(
            match_pattern("[^123]", "456"),
            Some("4".to_string()),
            "Test 6"
        );
        // assert_eq!(match_pattern("[^a-z]", "123"), true, "Extra_1");
        // assert_eq!(match_pattern("[^0-9]", "alpha"), true, "Extra_2");
    }

    #[test]
    fn test_stage_6() {
        assert_eq!(
            match_pattern("\\d apple", "1 apple"),
            Some("1 apple".to_string()),
            "Test 1"
        );
        assert_eq!(match_pattern("\\d apple", "1 orange"), None, "Test 2");
        assert_eq!(
            match_pattern("\\d\\d\\d apple", "100 apples"),
            Some("100 apple".to_string()),
            "Test 3"
        );
        assert_eq!(match_pattern("\\d\\d\\d apple", "1 apple"), None, "Test 4");
        assert_eq!(
            match_pattern("\\d \\w\\w\\ws", "3 dogs"),
            Some("3 dogs".to_string()),
            "Test 5"
        );
        assert_eq!(
            match_pattern("\\d \\w\\w\\ws", "4 cats"),
            Some("4 cats".to_string()),
            "Test 6"
        );
        assert_eq!(match_pattern("\\d \\w\\w\\ws", "1 dog"), None, "Test 7");
        assert_eq!(
            match_pattern("\\d\\w\\w\\w apple", "1dog apple"),
            Some("1dog apple".to_string()),
            "Test 8"
        );
    }

    #[test]
    fn test_stage_7() {
        assert_eq!(
            match_pattern("^log", "log"),
            Some("log".to_string()),
            "Test 1"
        );
        assert_eq!(match_pattern("^log", "slog"), None, "Test 2");
        assert_eq!(
            match_pattern("^apple", "apple pie"),
            Some("apple".to_string()),
            "Test 3"
        );
        assert_eq!(match_pattern("^apple", "pie apple"), None, "Test 4");
        assert_eq!(
            match_pattern("^123", "123456"),
            Some("123".to_string()),
            "Test 5"
        );
        assert_eq!(match_pattern("^123", "456123"), None, "Test 6");
        // assert_eq!(match_pattern("^ab", "abcd\nefgh"), true, "Test 7");
        // assert_eq!(match_pattern("^cd", "abcd\nefgh"), false, "Test 8");
    }

    #[test]
    fn test_stage_8() {
        assert_eq!(
            match_pattern("dog$", "dog"),
            Some("dog".to_string()),
            "Test 1"
        );
        assert_eq!(
            match_pattern("^dog$", "dog"),
            Some("dog".to_string()),
            "Test 1a"
        );
        assert_eq!(match_pattern("dog$", "dogs"), None, "Test 2");
        assert_eq!(
            match_pattern("pie$", "apple pie"),
            Some("pie".to_string()),
            "Test 3"
        );
        assert_eq!(match_pattern("^pie$", "apple pie"), None, "Test 3a");
        assert_eq!(
            match_pattern("apple$", "pie apple"),
            Some("apple".to_string()),
            "Test 4"
        );
        assert_eq!(match_pattern("apple$", "pie apple appl"), None, "Test 4a");
        assert_eq!(
            match_pattern("apple$", "pie apple apple"),
            Some("apple".to_string()),
            "Test 4b"
        );
        assert_eq!(match_pattern("123$", "123456"), None, "Test 5");
        assert_eq!(
            match_pattern("123$", "456123"),
            Some("123".to_string()),
            "Test 6"
        );
        assert_eq!(
            match_pattern("efgh$", "abcd\nefgh"),
            Some("efgh".to_string()),
            "Test 7"
        );
        assert_eq!(match_pattern("abcd$", "abcd\nefgh"), None, "Test 8");
    }
    #[test]
    fn test_stage_9() {
        assert_eq!(
            match_pattern("a+", "apple"),
            Some("a".to_string()),
            "Test 1"
        );
        assert_eq!(
            match_pattern("a+", "SaaS"),
            Some("aa".to_string()),
            "Test 2"
        );
        assert_eq!(match_pattern("a+", "dog"), None, "Test 3");
        assert_eq!(
            match_pattern("ca+ts", "cats"),
            Some("cats".to_string()),
            "Test 4"
        );
        assert_eq!(
            match_pattern("ca+ts", "caats"),
            Some("caats".to_string()),
            "Test 5"
        );
        assert_eq!(
            match_pattern("ca+ts", "caaaats"),
            Some("caaaats".to_string()),
            "Test 6"
        );
        assert_eq!(match_pattern("ca+ts", "ctss"), None, "Test 7");
        assert_eq!(
            match_pattern("ca+ts", "cass caats"),
            Some("caats".to_string()),
            "Test 8"
        );
        assert_eq!(match_pattern("^ca+ts", "cass caats"), None, "Test 9");
        assert_eq!(
            match_pattern("ca+ts$", "cass caats"),
            Some("caats".to_string()),
            "Test 9a"
        );
        assert_eq!(match_pattern("ca+ts$", "cats cass"), None, "Test 9b");
    }
    #[test]
    fn test_stage_10() {
        assert_eq!(
            match_pattern("a?", "apple"),
            Some("a".to_string()),
            "Test 1"
        );
        assert_eq!(match_pattern("a?", "SaaS"), Some("S".to_string()), "Test 2");
        assert_eq!(
            match_pattern("ca?ts", "cats"),
            Some("cats".to_string()),
            "Test 4"
        );
        assert_eq!(match_pattern("ca?ts", "caats"), None, "Test 5");
        assert_eq!(match_pattern("ca?ts", "caaaats"), None, "Test 6");
        assert_eq!(
            match_pattern("ca?ts", "cass cats"),
            Some("cats".to_string()),
            "Test 8"
        );
        assert_eq!(match_pattern("ca?ts", "cass caats"), None, "Test 9");
        assert_eq!(match_pattern("^ca?ts", "cass cats"), None, "Test 10");

        assert_eq!(match_pattern("a?", "dog"), Some("d".to_string()), "Test 3");
        assert_eq!(
            match_pattern("ca?ts", "cts"),
            Some("cts".to_string()),
            "Test 7"
        );
        assert_eq!(
            match_pattern("ca?ts", "cass cts"),
            Some("cts".to_string()),
            "Test 8a"
        );
    }
    #[test]
    fn test_stage_11() {
        assert_eq!(
            match_pattern("d.g", "dog"),
            Some("dog".to_string()),
            "Test 1"
        );
        assert_eq!(
            match_pattern("d.g", "dug"),
            Some("dug".to_string()),
            "Test 2"
        );
        assert_eq!(
            match_pattern("d.g", "d#g"),
            Some("d#g".to_string()),
            "Test 3"
        );
        assert_eq!(
            match_pattern("d.g", "dig"),
            Some("dig".to_string()),
            "Test 4"
        );
        assert_eq!(
            match_pattern(".at", "cat"),
            Some("cat".to_string()),
            "Test 5"
        );
        assert_eq!(
            match_pattern(".at", "bat"),
            Some("bat".to_string()),
            "Test 6"
        );
        assert_eq!(
            match_pattern(".at", "rat"),
            Some("rat".to_string()),
            "Test 7"
        );
        assert_eq!(
            match_pattern("b.t", "bat"),
            Some("bat".to_string()),
            "Test 8"
        );
        assert_eq!(
            match_pattern("b.t", "btt"),
            Some("btt".to_string()),
            "Test 9"
        );
        assert_eq!(match_pattern("b.t", "boot"), None, "Test 10");
        assert_eq!(
            match_pattern("...", "abc"),
            Some("abc".to_string()),
            "Test 11"
        );
        assert_eq!(
            match_pattern("...", "!@#"),
            Some("!@#".to_string()),
            "Test 13"
        );

        assert_eq!(
            match_pattern(".+", "apple"),
            Some("apple".to_string()),
            "Test 15"
        );
        assert_eq!(
            match_pattern(".+", "123"),
            Some("123".to_string()),
            "Test 16"
        );
        assert_eq!(
            match_pattern(".+", "!@#"),
            Some("!@#".to_string()),
            "Test 17"
        );

        assert_eq!(match_pattern("x.y", "xyz"), None, "Test 19");
        assert_eq!(
            match_pattern("x.y", "xty"),
            Some("xty".to_string()),
            "Test 20"
        );
        assert_eq!(
            match_pattern("x.y", "x1y"),
            Some("x1y".to_string()),
            "Test 21"
        );
        assert_eq!(match_pattern("x.y", "xy"), None, "Test 22");

        assert_eq!(
            match_pattern(".?at", "cat"),
            Some("cat".to_string()),
            "Test 23"
        );
        assert_eq!(
            match_pattern(".?at", "bat"),
            Some("bat".to_string()),
            "Test 24"
        );
        assert_eq!(
            match_pattern(".?at", "rrrat"),
            Some("rat".to_string()),
            "Test 25"
        );
        assert_eq!(match_pattern("^.?at", "rrrat"), None, "Test 25b");
        assert_eq!(
            match_pattern(".?at$", "rrrat"),
            Some("rat".to_string()),
            "Test 25c"
        );
        assert_eq!(
            match_pattern(".?at$", "ratat"),
            Some("tat".to_string()),
            "Test 25d"
        );
        assert_eq!(match_pattern(".?at$", "rataq"), None, "Test 25e");
        assert_eq!(
            match_pattern(".?at", "at"),
            Some("at".to_string()),
            "Test 26"
        );
        assert_eq!(match_pattern(".?at", "hhh"), None, "Test 27");
        assert_eq!(
            match_pattern("a.b.c.", "adbecf"),
            Some("adbecf".to_string()),
            "Test 28"
        );
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
