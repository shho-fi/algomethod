// https://algo-method.com/tasks/216

fn main() {
    let input = read_nums();
    let (n, v) = (input[0], input[1]);
    let target = read_nums()
        .into_iter()
        .enumerate()
        .rev()
        .find(|&(i, a)| a == v);
    println!("{}", target.unwrap_or((-1, 0)).0);
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
