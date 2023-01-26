// https://algo-method.com/tasks/229

fn main() {
    println!("{}", yes_or_no(read_line().contains(&read_line())));
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

fn yes_or_no(boolean: bool) -> String {
    (if boolean { "Yes" } else { "No" }).to_string()
}
