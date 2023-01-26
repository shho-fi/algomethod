// https://algo-method.com/tasks/33

fn main() {
    let s = read_line();
    let nums = read_nums();
    let n = nums[0] as usize;
    let m = nums[1] as usize;
    let mut chars: Vec<char> = s.chars().collect();
    chars.swap(n - 1, m - 1);
    println!("{}", chars.into_iter().collect::<String>());
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
