fn staircase(n: usize) {
    for i in 1..=n {
        let spaces = n - i;
        let hashes = i;
        println!("{}{}", " ".repeat(spaces), "#".repeat(hashes));
    }
}
fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");
    staircase(n);
}
