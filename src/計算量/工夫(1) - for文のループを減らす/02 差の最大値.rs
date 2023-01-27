// https://algo-method.com/tasks/1022

fn main() {
    read_line();
    let a = read_nums();
    let (max, min) = a.iter().fold((0i64, std::i64::MAX), |(max, min), x| {
        (max.max(*x), min.min(*x))
    });
    println!("{}", max - min);
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
