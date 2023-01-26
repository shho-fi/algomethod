// https://algo-method.com/tasks/30

fn main() {
    let nums = read_nums();
    println!("{}", nums.into_iter().min_by_key(|x| x % 10).unwrap());
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
