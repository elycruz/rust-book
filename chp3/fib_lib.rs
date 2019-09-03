/**
 * @reference https://www.goldennumber.net/math/
 */

static FIVE_O: f64 = 5.0;

static O_FIVE: f64 = 0.5;

// Phi, can also be expressed all in fives as below
// 5 ^ .5 * .5 + .5 = Φ
// (this is what we're doing below):
pub fn get_phi () -> f64 {
    FIVE_O.powf(O_FIVE) * O_FIVE + O_FIVE
}

pub fn fib_i64 (limit: i64) -> Vec<i64> {
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
    let divisor =  FIVE_O.powf(O_FIVE);
    if divisor > 0.0 && dividend > 0.0 {
        return (dividend / divisor).round() as i64;
    }
    return 0;
}
