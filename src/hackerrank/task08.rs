

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }
    let mut max = scores[0];
    let mut min = scores[0];
    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in scores.iter().skip(1) {
        if score > max {
            max = score;
            max_breaks += 1;
        } else if score < min {
            min = score;
            min_breaks += 1;
        }
    }
    vec![max_breaks, min_breaks]
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
        .expect("Invalid number of games");
    
    let scores_line = lines.next().unwrap().unwrap();
    let scores: Vec<i32> = scores_line
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid score"))
        .collect();
    
    assert_eq!(scores.len(), n);

    let result = breaking_records(&scores);
    println!("{} {}", result[0], result[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let scores1 = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&scores1), vec![2, 4]);

        let scores2 = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&scores2), vec![4, 0]);

        let scores3 = vec![12, 24, 10, 24];
        assert_eq!(breaking_records(&scores3), vec![1, 1]);
    }
}
