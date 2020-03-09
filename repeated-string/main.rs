use std::io;

fn main() {
    let s = readline();
    let n: usize = readline().parse().unwrap();

    println!("{}", repeated_string(&s, &n));
}

fn repeated_string(s: &str, n: &usize) -> usize {
    /*
        [[ EASY WAY BELOW, NOT PERFORMANT ]]

        let repeated = s.repeat(n / s.len() + 1);
        let sliced = &repeated[repeated.len() % n..];

        return sliced.chars().filter(|&c| c == 'a').count();
    */
    let flags = s.chars().map(|c| c == 'a');
    let enabledFlags = flags.filter(|&f| f).count();
    let length = flags.count();
    let total = (n / length + 1) * length;
    return 1;
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("could not read input");
    return input.trim().to_string();
}