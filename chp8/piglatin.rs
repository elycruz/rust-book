/**
 * Convert strings to pig latin.
 * The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay”.
 * Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
 * Keep in mind the details about UTF-8 encoding!
 */

use std::env;
use std::process;

const VOWELS: &str = "aeiou";
const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

//fn get_consonants() -> &str {
//    let start = u32::from('a');
//    let end = u32::from('z');
//    let mut i = start;
//    let mut out = String::new();
//    while i <= end {
//        let new_char = match std::char::from_u32(i) {
//            Some(x) => x,
//            _ => panic!("Error parsing u32 to char")
//        };
//        if VOWELS.contains(new_char.to_string().as_str()) {
//            out.push(new_char);
//        }
//        i += 1;
//    }
//    println!("{:?}", out);
//    return &out;
//}

fn word_to_piglatin(s: &str) -> String {
    if s.len() == 0 {
        return s.to_string();
    }
    let c0: char = match s.chars().next() {
        Some(x) => x,
        _ => panic!("Unable to get first char on non empty &str")
    };
    let mut out = String::new();
    for c in VOWELS.chars() {
        if c == c0 {
            out.push_str(s);
            out.push_str("hay");
            return out;
        }
    }
    for c in CONSONANTS.chars() {
        if c == c0 {
            out.push_str(&s[1..]);
            out.push(c0);
            out.push_str("ay");
            return out;
        }
    }
    return s.to_string();
}

fn line_to_piglatin(line: &str) -> String {
    let mut out = String::new();
    let words: Vec<&str> = line.split(' ').collect();
    let limit: usize = words.len() - 1;
    let mut i: usize = 0;
    for word in words  {
        out.push_str(word_to_piglatin(word).as_str());
        if i < limit {
            out.push(' ');
        }
        i += 1;
    }
    return out;
}

fn text_to_piglatin(text: &str) -> String {
    let mut out= String::new();
    let lines: Vec<&str> = text.lines().collect();
    let limit: usize = lines.len() - 1;
    let mut i: usize = 0;
    for line in lines {
        out.push_str(line_to_piglatin(line).as_str());
        if i < limit {
            out.push('\n');
        }
        i += 1;
    }
    return out;
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        eprintln!("Not enough args");
        process::exit(1);
    }

    args.next(); // skip first
    let text: String = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("No content received.");
            process::exit(1)
        }
    };

    println!("{:?}", text_to_piglatin(&text));
}

#[cfg(test)]
mod test {
    use crate::{word_to_piglatin, line_to_piglatin, text_to_piglatin};

    #[test]
    fn test_word_to_piglatin() {
        for (control, expected) in vec![
            ("hello", "ellohay"),
            ("abc", "abchay"),
            ("road", "oadray"),
            ("first", "irstfay")
        ] {
            println!("word_to_piglatin({:?})", control);
            let result = word_to_piglatin(control);
            assert_eq!(result.as_str(), expected);
        }
    }

    #[test]
    fn test_line_to_piglatin() {
        for (control, expected) in vec![
            ("hello", "ellohay"),
            ("hello world", "ellohay orldway"),
            ("abc today", "abchay odaytay"),
            ("all your base are belong to us", "allhay ouryay asebay arehay elongbay otay ushay"),
        ] {
            println!("line_to_piglatin({:?})", control);
            let result = line_to_piglatin(control);
            assert_eq!(result.as_str(), expected);
        }
    }

    #[test]
    fn test_to_piglatin() {
        for (control, expected) in vec![
            ("hello", "ellohay"),
            ("hello world", "ellohay orldway"),
            ("abc today", "abchay odaytay"),
            ("abc\n today", "abchay\n odaytay"),
            ("all your base are belong to us", "allhay ouryay asebay arehay elongbay otay ushay"),
            ("all your\n base are\n belong to us", "allhay ouryay\n asebay arehay\n elongbay otay ushay"),
        ] {
            println!("text_to_piglatin({:?})", control);
            let result = text_to_piglatin(control);
            assert_eq!(result.as_str(), expected);
        }
    }
}
