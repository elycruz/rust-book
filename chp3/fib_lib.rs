/**
 * @reference https://www.goldennumber.net/math/
 */

static FIVE_O: f64 = 5.0;

static O_FIVE: f64 = 0.5;

// Phi, can also be expressed all in fives as below
// 5 ^ .5 * .5 + .5 = Φ
// (this is what we're doing below):
fn get_phi() -> f64 {
    FIVE_O.powf(O_FIVE) * O_FIVE + O_FIVE
}

pub fn fib_i64(limit: i64) -> Vec<i64> {
    let mut out: Vec<i64> = Vec::new();
    let mut a = 0;
    let mut b = 1;
    while a <= limit {
        out.push(a);
        if b <= limit {
            out.push(b);
        }
        a = a + b;
        b = a + b;
    }
    out
}

pub fn nth_fib(nth: i32) -> i64 {
    let dividend = get_phi().powi(nth);
    let divisor = FIVE_O.powf(O_FIVE);
    if divisor > 0.0 && dividend > 0.0 {
        return (dividend / divisor).round() as i64;
    }
    return 0;
}

#[cfg(test)]
mod test {
    use crate::fib_lib::{nth_fib, fib_i64};

    #[test]
    fn test_nth_fib() {
        let test_cases:Vec<i64> = vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377
        ];
        let mut i = 0;
        for n in test_cases.iter() {
            println!("nth_fib({:})", i);
            let result = nth_fib(i);
            println!("Expect {:} to equal {:}", result, *n);
            assert_eq!(result, *n);
            i += 1;
        }
    }

    #[test]
    fn test_fib_i64() {
        let test_cases:Vec<i64> = vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377
        ];

        for n in test_cases.iter() {
            println!("fib_i64({:})", *n);

            let result = fib_i64(*n);

            let mut i = 0;

            for n2 in result.iter() {
                println!("Expect {:?} to equal {:}", *n2, test_cases[i]);
                assert_eq!(*n2, test_cases[i]);
                i += 1;
            }
        }
    }
}
