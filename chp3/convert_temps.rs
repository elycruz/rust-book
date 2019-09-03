// Temperatures math reference: https://www.mathsisfun.com/temperature-conversion.html

pub fn to_celsius(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

pub fn to_farenheit(x: f32) -> f32 {
    (x * 9.0) / 5.0 + 32.0
}

pub fn get_test_cases() -> Vec<(f32, f32)> {
    // [(Celsius, Farenheit)]
    vec![
        (180.0, 356.0),
        (100.0, 212.0),
        (40.0, 104.0),
        (37.0, 98.6),
        (30.0, 86.0),
//        (21.0, 70.0),   // estimated number
        (10.0, 50.0),
        (0.0, 32.0),
//        (-18.0, 0.0),   // estimated number
        (-40.0, -40.0),
    ]
}

fn main() {
    println!("\nto_celsius(Farenheit) -> Celsius\n");
    for (_, faren) in get_test_cases().iter() {
        println!("to_celsius({:}) -> {:}", *faren, to_celsius(*faren));
    }
    println!("\nto_farenheit(Celsius) -> Farenheit\n");
    for (_cels, _) in get_test_cases().iter() {
        println!("to_farenheit({:}) -> {:}", _cels, to_farenheit(*_cels));
    }
    println!(" ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_farenheit() {
        for (_cels, _faren) in get_test_cases().iter() {
            println!("to_farenheit({:})", *_cels);
            assert_eq!(to_farenheit(*_cels), *_faren);
        }
    }

    #[test]
    fn test_to_celsius() {
        for (_cels, _faren) in get_test_cases().iter() {
            println!("to_celsius({:})", *_faren);
            assert_eq!(to_celsius(*_faren), *_cels);
        }
    }
}
