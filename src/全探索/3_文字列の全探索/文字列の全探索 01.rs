// https://algo-method.com/tasks/226

fn main() {
    let s = read_line();
    let c = read_line();
    println!("{}", if s.contains(&c) { "Yes" } else { "No" });
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
