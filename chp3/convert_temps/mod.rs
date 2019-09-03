// Temperatures math reference: https://www.mathsisfun.com/temperature-conversion.html

pub fn to_farenheit<T>(x: T) -> f32 {
    f32(x) - 32.0 * 5.0 / 9.0
}

pub fn to_celsius<T>(x: T) -> f32 {
    f32(x) * 9.0 / 5.0 + 5.0
}
