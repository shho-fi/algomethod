// https://algo-method.com/tasks/238

fn main() {
    let line = read_nums();
    let (l, r) = (line[0], line[1]);
    println!(
        "{}",
        (l..=r)
            .filter(|x| {
                let s = x.to_string();
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
