// https://algo-method.com/tasks/32

fn main() {
    println!(
        "{}",
        read_line()
            .chars()
            .nth((read_nums()[0] - 1) as usize)
            .unwrap()
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
