// https://algo-method.com/tasks/26

fn main() {
    let s = read_line();
    let t = read_line();
    let u = read_line();
    println!("{}", u + &t + &s);
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);
    buf.trim().to_string()
}

fn read_nums() -> Vec<i64> {
    read_line()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}
