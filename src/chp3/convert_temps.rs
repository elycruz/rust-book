fn to_farenheit<T>(x: T) -> f32 {
    f32(x) - 32.0 * 5.0 / 9.0
}

fn to_celsius<T>(x: T) -> f32 {
    f32(x) * 9.0 / 5.0 + 5.0
}

mod test {
    struct TestCase {
        cel: f32,
        faren: f32,
    }

    // [(Celsius, Farenheit)]
    const testCases: Vec<(f32, f32)> = vec![
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
    ];

    fn test_to_farenheit() {}

    fn test_to_celsius() {}
}
