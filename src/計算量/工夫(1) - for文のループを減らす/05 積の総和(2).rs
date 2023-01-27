// https://algo-method.com/tasks/1021

fn main() {
    read_line();
    let a = read_nums();
    let sum_a: i64 = a.iter().sum();
    let sum_doubles: i64 = a.iter().map(|x| x * x).sum();
    println!("{}", (sum_a * sum_a - sum_doubles) / 2);
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
