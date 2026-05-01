
fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter().filter(|&&d| {
        let pos = a + d;
        pos >= s && pos <= t
    }).count();

    let orange_count = oranges.iter().filter(|&&d| {
        let pos = b + d;
        pos >= s && pos <= t
    }).count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let s: i32 = iter.next().unwrap().parse().unwrap();
    let t: i32 = iter.next().unwrap().parse().unwrap();
    let second_line = lines.next().unwrap().unwrap();
    let mut iter = second_line.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let third_line = lines.next().unwrap().unwrap();
    let mut iter = third_line.split_whitespace();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let apples_line = lines.next().unwrap().unwrap();
    let apples: Vec<i32> = apples_line
        .split_whitespace()
        .take(m)
        .map(|x| x.parse().unwrap())
        .collect();
    let oranges_line = lines.next().unwrap().unwrap();
    let oranges: Vec<i32> = oranges_line
        .split_whitespace()
        .take(n)
        .map(|x| x.parse().unwrap())
        .collect();

    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];
        let mut apple_count = 0;
        let mut orange_count = 0;
        for &d in &apples {
            let pos = 5 + d;
            if pos >= 7 && pos <= 11 { apple_count += 1; }
        }
        for &d in &oranges {
            let pos = 15 + d;
            if pos >= 7 && pos <= 11 { orange_count += 1; }
        }
        assert_eq!(apple_count, 1);
        assert_eq!(orange_count, 1);
    }
}
