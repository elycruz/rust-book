pub fn head_after_trim_and_to_lower(xs: &String) -> Option<char> {
    xs.trim().to_lowercase().as_str().chars().next()
}

pub fn starts_with_yes_no_char(xs: &String) -> (bool, char) {
    let chr = head_after_trim_and_to_lower(xs);
    match chr {
        Some(x) => match x {
            'y' | 'n' => (true, x),
            _ => (false, x),
        }
        None => (true, ' ')
    }
}

pub fn yes_no_to_bool(c: char) -> bool {
    match c {
        'y' | ' ' => true,
        _ => false
    }
}

pub fn normalize_num_guess(g: String) -> i32 {
    let num: i32 = g.trim().parse().expect("Please enter a number:");
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_after_trimmed_and_to_lower() {
        let v = vec![
            ("y", 'y'),
            ("Y", 'y'),
            ("", ' '), // "no char" is represented by ' ' in our app
            ("n", 'n'),
            ("N", 'n'),
            ("w", 'w'),
            ("99", '9'),
            ("-1", '-'),
            ("0", '0'),
            ("1", '1'),
        ];

        for t in &v {
            println!("head_trimmed_to_lower({:})", t.0);
            let result = match head_after_trim_and_to_lower(&String::from(t.0)) {
                Some(x) => x,
                _ => ' '
            };
            println!("Expect {:};  Received {:}", t.1, result);
            assert_eq!(t.1, result);
        }
    }

    #[test]
    fn test_starts_with_yes_no_char() {
        let v = vec![
            ("y", true),
            ("Y", true),
            ("", true), // "no char", is represented by ' ' in our app
            ("n", true),
            ("N", true),
            ("w", false),
            ("99", false),
            ("-1", false),
            ("0", false),
            ("1", false),
        ];

        for t in &v {
            println!("starts_with_yes_no_char({:}) == {:?}", t.0, t);

            let chr_str = &String::from(t.0);
            let (bln, chr) = starts_with_yes_no_char(chr_str);

            println!("Expect {:?} to equal {:}", bln, t.1);
            assert_eq!(bln, t.1);

            let expectation: bool;
            if bln {
                println!("When result is `(true, char)` expect char to equal one of ['y','n', ' '].");
                expectation = chr == 'y' || chr == 'n' || chr == ' ';
                assert_eq!(expectation, bln);
            } else {
                let c = head_after_trim_and_to_lower(&String::from(t.0)).unwrap();
                println!("When result is `(false, char)` expect {:} to equal {:}", chr, c);
                assert_eq!(chr, c);
            }
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
        for (s, b) in &v {
            let subj = match head_after_trim_and_to_lower(&String::from(*s)) {
                Some(x) => x,
                _ => ' '
            };
            println!("yes_no_to_bool({:}) == {:}", *s, *b);
            assert_eq!(yes_no_to_bool(subj), *b);
        }
    }
}
