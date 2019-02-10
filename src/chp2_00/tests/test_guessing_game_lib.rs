#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_is_yes_no_bool() {
        let v = vec![
            ("y", true),
            ("Y", true),
            ("", true),
            ("n", true),
            ("N", true),
            ("w", false),
            ("99", false),
            ("-1", false),
            ("0", false),
            ("1", false),
        ];
        for t in &v {
            assert_eq!(is_yes_no_bool(t[0]), t[1]);
        }
    }

    #[test]
    fn test_yes_no_to_bool() {
        let v = vec![
            ("y", true),
            ("Y", true),
            ("", true),
            ("n", false),
            ("N", false),
            ("w", false),
        ];
        for t in &v {
            assert_eq!(yes_no_to_bool(t[0]), t[1]);
        }
    }
}
