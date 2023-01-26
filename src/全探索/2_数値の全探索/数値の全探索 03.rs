// https://algo-method.com/tasks/222

fn main() {
    let n = read_nums()[0];
    let n_sqrt = (n as f64).sqrt() as i64;
    println!(
        "{}",
        if n == 1 || (2..=n_sqrt).any(|x| n % x == 0) {
            "No"
        } else {
            "Yes"
        }
    );
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
