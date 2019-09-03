use std::string::String;

// `ucase_first` credit to Shepmaster in stackoverflow thread: https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust/53571882#53571882
fn ucase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn get_day_line(day: &str) -> String {
    format!("On the {:} day of Christmas my true love sent to me:", day)
}

fn main() {
    let days = [
        "first", "second", "third", "fourth",
        "fifth", "sixth", "seventh", "eight",
        "ninth", "tenth", "eleventh", "twelfth"
    ];

    let items = [
        "twelve Lords a leaping",
        "eleven ladies dancing",
        "ten pipers piping",
        "nine, drummers drumming",
        "eight maids a milking",
        "seven swans a swimming",
        "six geese a laying",
        "and five gold rings",
        "four calling birds",
        "three French hens",
        "two turtle doves",
        "a partridge in a pear tree"
    ];

    let days_len = days.len();
    let mut days_index = days_len;

    for day in &days {
        days_index -= 1;
        println!("\n{:}", get_day_line(day));

        let items_for_day = &items[days_index..days_len];
        let items_for_day_len = items_for_day.len();
        let mut i: usize = items_for_day_len;

        // If more than one item for day
        if i > 1 {
            // Loop through items and capitalize the first one and add "and " to last one.
            for line in items_for_day {
                i -= 1;
                if i == items_for_day_len - 1 {
                    println!("{:}", ucase_first(*line));
                }
                else if i == 0 {
                    println!("and {:}", line);
                }
                else {
                    println!("{:}", line);
                }
            }

            // Print last line again if final day
            if items_for_day_len == 12 {
                println!("and {:}", items_for_day[items_for_day_len - 1])
            }
        } else {
            println!("{:}", ucase_first(items_for_day[0]));
        }
    }
}
