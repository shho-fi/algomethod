// https://algo-method.com/tasks/28

fn main() {
    let nums = read_nums();
    let cond = nums[0] % nums[1] == 0;
    println!("{}", if cond { "Yes" } else { "No" });
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
