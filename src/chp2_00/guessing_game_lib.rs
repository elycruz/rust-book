pub fn string_to_yes_no_char(xs: &String) -> String {
    xs.trim().to_lowercase()
}

pub fn is_yes_no_char(xs: &String) -> bool {
    let answer = string_to_yes_no_char(xs);
    let slice: &str = answer.as_str();
    match slice {
        "y" | "n" | "" => true,
        _ => false
    }
}

pub fn yes_no_to_bool(xs: &str) -> bool {
    match xs {
        "y" | "" => true,
        "n" | _ => false
    }
}

pub fn normalize_num_guess(g: String) -> i32 {
    let num: i32 = g.trim().parse().expect("Please enter a number:");
    return num;
}
