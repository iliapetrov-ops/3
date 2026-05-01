
fn diagonal_difference(matrix: &[Vec<i32>]) -> i32 {
    let n = matrix.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;
    
    for i in 0..n {
        primary_sum += matrix[i][i];
        secondary_sum += matrix[i][n - 1 - i];
    }
    
    (primary_sum - secondary_sum).abs()
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
        .expect("Invalid matrix size");

    let mut matrix = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();
        assert_eq!(row.len(), n);
        matrix.push(row);
    }

    let result = diagonal_difference(&matrix);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        let matrix = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonal_difference(&matrix), 15);
    }
}
