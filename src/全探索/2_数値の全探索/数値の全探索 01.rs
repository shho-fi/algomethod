// https://algo-method.com/tasks/220

fn main() {
    let n = read_nums()[0];
    println!(
        "{}",
        (1..=n)
            .filter(|x| x % 2 != 0 && x % 3 != 0 && x % 5 != 0)
            .count()
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
