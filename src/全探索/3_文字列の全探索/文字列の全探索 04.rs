// https://algo-method.com/tasks/230

fn main() {
    read_line();
    let (s, t) = (read_line(), read_line());
    println!(
        "{}",
        s.chars().zip(t.chars()).filter(|(x, y)| x != y).count()
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
