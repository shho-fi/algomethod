// https://algo-method.com/tasks/210

fn main() {
    let v = read_nums()[1];
    println!("{}", read_nums().into_iter().filter(|&a| a == v).count());
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
