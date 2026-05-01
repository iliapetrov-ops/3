
fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade < 38 {
                grade
            } else {
                let next_multiple = ((grade / 5) + 1) * 5;
                if next_multiple - grade < 3 {
                    next_multiple
                } else {
                    grade
                }
            }
        })
        .collect()
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
        .expect("Invalid number of students");

    let mut grades = Vec::with_capacity(n);
    for _ in 0..n {
        let grade: i32 = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse()
            .expect("Invalid grade");
        grades.push(grade);
    }

    let rounded = grading_students(&grades);

    for grade in rounded {
        println!("{}", grade);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading() {
        let grades = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&grades), expected);
    }
}
