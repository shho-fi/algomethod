// https://algo-method.com/tasks/234

fn main() {
    read_line();
    println!(
        "{}",
        read_nums().into_iter().filter(|&a| is_prime(a)).count()
    );
}

fn is_prime(num: i64) -> bool {
    num != 1 && (2..=((num as f64).sqrt() as i64)).all(|n| num % n != 0)
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
