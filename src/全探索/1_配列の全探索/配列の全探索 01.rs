// https://algo-method.com/tasks/209

fn main() {
    let input = read_nums();
    let cond = read_nums().iter().any(|&a| a == input[1]);
    println!("{}", if cond { "Yes" } else { "No" });
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
