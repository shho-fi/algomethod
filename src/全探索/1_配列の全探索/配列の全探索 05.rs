// https://algo-method.com/tasks/215

fn main() {
    read_line();
    println!("{}", read_nums().windows(2).filter(|a| a[0] < a[1]).count());
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
