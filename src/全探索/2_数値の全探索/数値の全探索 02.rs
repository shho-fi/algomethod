// https://algo-method.com/tasks/221

fn main() {
    let n = read_nums()[0];
    let n_sqrt = (n as f64).sqrt() as i64;
    println!(
        "{}",
        (1..=n_sqrt)
            .filter(|&x| n % x == 0)
            .map(|x| if n / x > n_sqrt { 2 } else { 1 })
            .sum::<i64>()
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
