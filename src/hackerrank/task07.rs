
fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = a.iter().fold(1, |lcm, &x| lcm * x / gcd(lcm, x));
    let gcd_b = b.iter().fold(0, |gcd, &x| gcd_num(gcd, x));
    
    let mut count = 0;
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }
    count
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

fn gcd_num(a: i32, b: i32) -> i32 {
    gcd(a, b)
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nm = first_line.split_whitespace();
    let n: usize = nm.next().unwrap().parse().expect("Invalid n");
    let m: usize = nm.next().unwrap().parse().expect("Invalid m");

    let second_line = lines.next().unwrap().unwrap();
    let a: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number in a"))
        .collect();
    assert_eq!(a.len(), n);

    let third_line = lines.next().unwrap().unwrap();
    let b: Vec<i32> = third_line
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number in b"))
        .collect();
    assert_eq!(b.len(), m);

    let result = get_total_x(&a, &b);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_another() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq!(get_total_x(&a, &b), 2);
    }
}
