use std::io;

fn main() {
    let s = readline();
    let n: usize = readline().parse().unwrap();

    println!("{}", repeated_string(&s, &n));
}

fn repeated_string(s: &str, n: &usize) -> usize {
    let length = s.len();
    let enabled_flags = s.chars().filter(|&c| c == 'a').count();

    let repetitions = n / length;
    let excess = n % length;
    let result = s.chars().take(excess).filter(|c| *c == 'a').count();

    return result + repetitions * enabled_flags;
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("could not read input");
    return input.trim().to_string();
}