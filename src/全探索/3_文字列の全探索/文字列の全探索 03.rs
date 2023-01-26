// https://algo-method.com/tasks/228

fn main() {
    println!(
        "{}",
        read_line()
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .filter(|sl| sl[0] == sl[1])
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
