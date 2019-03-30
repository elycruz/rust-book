use std::string::String;

fn main() {
    let one_to_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print_in_vec(&one_to_ten);
    print_in_vec_and_increase_by_one(&one_to_ten);
    print_chars_seq('a', 'z');
    println!("\nPrint chars sequence from 'a' to 'z':");
    println!("{:}", get_chars_seq('a', 'z'));
}

fn print_in_vec(v: &Vec<i32>) {
    println!("\nPrinting items in vec.");
    for x in v {
        print!("{:}", x);
    }
}

fn print_in_vec_and_increase_by_one(v: &Vec<i32>) {
    println!("\nPrinting items in vec.");
    let mut out: Vec<i32> = vec![];
    for x in v {
        out.push(x + 1);
    }
    println!("{:?}", out);
}

fn print_chars_seq(a: char, b: char) {
    println!("\nPrint chars sequence from '{:}' to '{:}':\n", a, b);
    for c in (a as u8)..=(b as u8) {
        print!("{}", c as char);
    }
}

fn get_chars_seq(a: char, b: char) -> String {
    let mut out = String::new();
    for c in (a as u8)..=(b as u8) {
        out.push(c as char);
    }
    return out;
}

fn maximum<T: Ord + std::fmt::Debug>(xs: &Vec<T>) -> Option<&T> {
    if xs.len() == 0 {
        return None;
    }
    let mut last = &xs[0];
    for x in xs.iter() {
        if x.gt(last) {
            last = x;
        }
    }
    return Some(last);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum() {
        let tests_table: Vec<(&str, Vec<i32>, i32)> = vec![
            ("maximum 0-9 == 9", (0..=9).collect(), 9),
            ("maximum 21-55 == 55", (21..=55).collect(), 55)
        ];
        for tc in tests_table {
            let (name, range, expected) = tc;
            let xs: Vec<i32> = range;
            println!("{:}", name);
            assert_eq!(maximum(&xs).unwrap(), &expected);
        }
    }
}
