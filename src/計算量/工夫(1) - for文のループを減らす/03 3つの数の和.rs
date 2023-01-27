// https://algo-method.com/tasks/1024

fn main() {
    let line = read_nums();
    let (n, m) = (line[0], line[1]);
    println!(
        "{}",
        (1..=n)
            .flat_map(|i| {
                (1..=n)
                    .filter(move |j| i + j < m)
                    .map(move |j| n.min(m - i - j))
            })
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
