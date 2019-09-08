use std::collections::HashMap;

fn mean(v: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for x in v {
        sum += *x;
    }
    return sum / (v.len() as f64);
}

fn mode(v: &Vec<f64>) -> f64 {
    let mut collected: HashMap<&str, usize> = HashMap::new();
    for k in v.iter() {
        let key = format!("{:?}", *k).as_str();
        *collected.entry(key).or_insert(0) += 1;
    }
    let mut max: (&str, usize) = ("0.0", 0);
    for (k, v) in collected.iter() {
        if *v > max.1 {
            max = (*k, *v);
        }
    }
    let r: Result<f64, _> = str::parse(max.0);
    match r {
        Ok(x) => x,
        Err(e) => panic!(e)
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::mean;

    #[test]
    fn test_mean() {
        let cases:Vec<(Vec<f64>, f64)> = vec![
            (vec![1.0, 2.0, 3.0, 4.0, 5.0], 3.0),
            (vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], 5.5),
            (vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0], 7.0),
        ];
        for (arg, expected) in cases.iter() {
            println!("mean({:?})", arg);
            let r = mean(arg);
            println!("Result: {:?}", r);
            assert_eq!(r, *expected);
        }
    }

    #[test]
    fn test_mode() {}
}
