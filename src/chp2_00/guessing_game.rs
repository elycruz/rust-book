use std::io;
use std::cmp::Ordering;
use std::string::String;
use rand;
use rand::Rng;

extern crate regex;

mod guessing_game_lib;

use regex::Regex;
use guessing_game_lib::*;

static ALLOWED_NUM_GUESSES: i32 = 3;

fn ask_for_guess (re: &Regex) -> String {
    let mut guess = String::new();
    while !re.is_match(guess.as_str()) {
        println!("Please input your guess:");
        guess.clear();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    }
    guess
}

fn ask_play_again (yes_no_regex: &Regex) -> bool {
    let mut is_valid_rslt = false;
    let mut answer = String::new();
    let mut chr: char = ' ';
    while !is_valid_rslt {
        println!("Play again y/n?(y)");
        io::stdin().read_line(&mut answer)
            .expect("Failed to read line");
        let (result, char_str) = starts_with_yes_no_char(&answer);
        is_valid_rslt = result;
        chr = char_str;
        if !is_valid_rslt {
            answer.clear();
        }
    }
    yes_no_to_bool(chr)
}

fn get_guess(re: &Regex) -> i32 {
    normalize_num_guess(
        ask_for_guess(re)
    )
}

fn play_game(num_regex: &Regex) {
    println!("Guess the number (between 0-9):");
    let mut remaining_num_guesses: i32 = ALLOWED_NUM_GUESSES;
    let secret_num: i32 = rand::thread_rng().gen_range(0, 9);
    let mut is_correct_guess: bool = false;
    while is_correct_guess == false && remaining_num_guesses > 0 {
        let guess: i32 = get_guess(num_regex);
        let (guess_bln, guess_msg) = match guess.cmp(&secret_num) {
            Ordering::Less => (false, "Too small."),
            Ordering::Greater => (false, "Too large."),
            Ordering::Equal => (true, "You win!"),
        };
        remaining_num_guesses -= 1;
        is_correct_guess = guess_bln;
        println!("{}", guess_msg);
    }
}

fn main () {
    let mut quit_game = false;
    let num_regex: &Regex = &match regex::Regex::new(r"\d{1,3}") {
        Err(err) => panic!("{}", err),
        Ok(x) => x
    };
    let _yes_no_regex: &Regex = &match regex::Regex::new(r"(?:y|n)") {
        Err(err) => panic!("{}", err),
        Ok(x) => x
    };
    while quit_game == false {
        play_game(num_regex);
        quit_game = !ask_play_again(_yes_no_regex);
    }
}

