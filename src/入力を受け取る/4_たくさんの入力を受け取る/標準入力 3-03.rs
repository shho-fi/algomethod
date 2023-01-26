// https://algo-method.com/tasks/52

fn main() {
    read_line();
    read_nums().iter().for_each(|a| println!("{}", a % 10));
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
