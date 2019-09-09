use std::collections::HashMap;

fn mean(v: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for x in v {
        sum += *x;
    }
    return sum / (v.len() as f64);
}

fn mode(vs: &Vec<f64>) -> f64 {
    let mut v_count_map: HashMap<String, usize> = HashMap::new();

    // Collect counts per item in vectors
    for k in vs {
        let v_ptr = v_count_map.
            entry(k.to_string()).
            or_insert(0)
            ;
        *v_ptr += 1;
    }

    let mut max: (&str, usize) = ("0.0", 0);
    let mut keys: Vec<&String> = v_count_map.keys().collect();

    keys.sort();

    println!("Key counts {:?}", v_count_map);
    println!("Ordered map keys {:?}", keys);

    for k in keys.iter() {
        match v_count_map.get(*k) {
            Some(v) => {
                if *v > max.1 {
                    max = (*k, *v);
                }
            }
            _ => println!("{:?} doesn't exist in map keys {:?}", k, keys)
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
    use super::*;

    #[test]
    fn test_mean() {
        let cases: Vec<(Vec<f64>, f64)> = vec![
            (vec![1.0, 2.0, 3.0, 4.0, 5.0], 3.0),
            (vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], 5.5),
            (vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0], 7.0),
        ];
        for (arg, expected) in cases.iter() {
            println!("mean({:?})", arg);
            let r = mean(arg);
            assert_eq!(r, *expected);
        }
    }

    #[test]
    fn test_mode() {
        let cases: Vec<(Vec<f64>, f64)> = vec![
            (vec![], 0.0),
            (vec![1.0, 2.0, 3.0, 5.0, 1.0, 2.0, 2.0, 3.0, 3.1, 3.0, 1.0], 1.0), // 2.0 reaches a count of three before 1.0
            (vec![1.0, 2.0, 3.0, 5.0, 2.0, 8.0], 2.0),
            (vec![1.0, 2.0, 3.0, 5.0, 3.0, 8.0, 13.0], 3.0),
            (vec![1.0, 2.0, 3.0, 5.0, 8.0, 8.1, 13.0, 13.1], 1.0),
        ];
        for (arg, expected) in cases.iter() {
            println!("mode({:?})", arg);
            let r = mode(arg);
            assert_eq!(r, *expected);
        }
    }
}
