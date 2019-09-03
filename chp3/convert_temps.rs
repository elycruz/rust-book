// Temperatures math reference: https://www.mathsisfun.com/temperature-conversion.html

pub fn to_farenheit(x: f32) -> f32 {
    x - 32.0 * 5.0 / 9.0
}

pub fn to_celsius(x: f32) -> f32 {
    x * 9.0 / 5.0 + 5.0
}

pub fn get_test_cases() -> Vec<(f32, f32)> {
    // [(Celsius, Farenheit)]
    vec![
        (180.0, 356.0), //  Moderate Oven
        (100.0, 212.0), //  Water boils
        (40.0, 104.0),  //  Hot Bath
        (37.0, 98.6),   //  Body temperature
        (30.0, 86.0),   //  Beach weather
        (21.0, 70.0),   //  Room temperature
        (10.0, 50.0),   //  Cool Day
        (0.0, 32.0),    //  Freezing point of water
        (-18.0, 0.0),   //  Very Cold Day
        (-40.0, -40.0),
    ]
}

fn main() {
    println!("Convert celsius from farenheit");
    for (cels, faren) in get_test_cases().iter() {
        println!("to_celsius({:}) -> {:}", to_celsius(*faren), faren);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_to_farenheit() {
        for (_cels, _faren) in get_test_cases().iter() {
            assert_eq!(to_farenheit(*_cels), *_faren);
        }
    }

    #[test]
    fn test_to_celsius() {
        for (_cels, _faren) in get_test_cases().iter() {
            assert_eq!(to_celsius(*_faren), *_cels);
        }
    }
}
