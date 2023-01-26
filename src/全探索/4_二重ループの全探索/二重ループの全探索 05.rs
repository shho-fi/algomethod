// https://algo-method.com/tasks/237

fn main() {
    let n = read_nums()[0];
    println!(
        "{}",
        (0..n)
            .filter(|_| {
                let s = read_line();
                s == s.chars().rev().collect::<String>()
            })
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
