
fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        return if x1 == x2 { "YES".to_string() } else { "NO".to_string() };
    }
    let diff_x = x2 - x1;
    let diff_v = v1 - v2;
    if diff_x % diff_v == 0 && diff_x / diff_v >= 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    
    let x1 = nums[0];
    let v1 = nums[1];
    let x2 = nums[2];
    let v2 = nums[3];
    
    println!("{}", kangaroo(x1, v1, x2, v2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
        assert_eq!(kangaroo(0, 5, 5, 5), "NO");
        assert_eq!(kangaroo(5, 1, 5, 2), "NO");
        assert_eq!(kangaroo(5, 2, 5, 1), "YES");
        assert_eq!(kangaroo(21, 6, 47, 3), "NO");
        assert_eq!(kangaroo(43, 2, 70, 2), "NO");
    }
}
