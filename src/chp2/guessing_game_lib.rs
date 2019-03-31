pub fn to_yes_no_char_option(xs: &String) -> Option<char> {
    xs.trim().to_lowercase().as_str().chars().next()
}

pub fn starts_with_yes_no_char(xs: &String) -> (bool, char) {
    let chr = to_yes_no_char_option(xs);
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
