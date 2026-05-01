
fn migratory_birds(arr: &[i32]) -> i32 {
    let mut freq = [0; 6];
    for &bird in arr {
        freq[bird as usize] += 1;
    }
    let mut max_freq = 0;
    let mut result = 0;
    for bird_type in 1..=5 {
        if freq[bird_type] > max_freq {
            max_freq = freq[bird_type];
            result = bird_type;
        } else if freq[bird_type] == max_freq && bird_type < result {
            result = bird_type;
        }
    }
    result
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid number of sightings");

    let birds_line = lines.next().unwrap().unwrap();
    let birds: Vec<i32> = birds_line
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid bird type"))
        .collect();

    assert_eq!(birds.len(), n);

    let result = migratory_birds(&birds);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);
        assert_eq!(migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]), 3);
        assert_eq!(migratory_birds(&[1, 1, 2, 2, 3]), 1);
        assert_eq!(migratory_birds(&[5, 5, 5, 5, 1]), 5);
    }
}
