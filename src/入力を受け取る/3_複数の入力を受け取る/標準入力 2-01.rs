// https://algo-method.com/tasks/24

fn main() {
    let numbers: Vec<i32> = read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let a = numbers[0];
    let b = numbers[1];
    println!("{}", a + b);
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);
    buf.trim().to_string()
}
