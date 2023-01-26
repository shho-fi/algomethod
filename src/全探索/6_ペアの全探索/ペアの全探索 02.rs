// https://algo-method.com/tasks/245

fn main() {
    let line = read_nums();
    let (l, r) = (line[0], line[1]);
    println!(
        "{}",
        (l..r)
            .flat_map(|i| (l..=r)
                .filter(move |j| i < *j && i % 10 == *j % 10)
                .map(move |j| (i, j)))
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
