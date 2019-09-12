use std::env;
use std::process;

const VOWELS: &str = "aeiou";
const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

//fn get_consonants() -> &str {
//    let start = u32::from('a');
//    let end = u32::from('z');
//    let mut i = start;
//    let mut out = String::from("");
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

/**
 * Convert strings to pig latin.
 * The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay”.
 * Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
 * Keep in mind the details about UTF-8 encoding!
 */
fn word_to_piglatin<'a>(s: &str) -> String {
    if s.len() == 0 {
        return s.to_string();
    }
    let c0: char = match s.chars().next() {
        Some(x) => x,
        _ => panic!("Unable to get first char on non empty &str")
    };
    let mut out: String = String::from("");
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

fn line_to_piglatin(line: &str) -> &str {
    let words: Vec<&str> = line.split(' ').collect();
    let mut out: Vec<&str> = vec![];
    for word in words {
        let transformed_word = word_to_piglatin(word);
        out.push(transformed_word.as_str());
    }
    return out.join(" ").as_str();
}

fn lines_to_piglatin(lines: Vec<&str>) -> &str {
    let mut out: String = String::from("");
    for line in lines {
        out.push_str(line_to_piglatin(line));
    }
    return &out;
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

    let split_text: Vec<&str> = text.split(' ').collect();
    println!("{:?}", lines_to_piglatin(split_text));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_word_to_piglatin() {}

    #[test]
    fn test_line_to_piglatin() {}
}
