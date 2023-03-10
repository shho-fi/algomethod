// https://algo-method.com/tasks/58

fn main() {
    let n = read_nums()[0];
    println!(
        "{}",
        (0..n)
            .map(|_| read_line().chars().nth(0).unwrap())
            .collect::<String>()
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
