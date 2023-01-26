// https://algo-method.com/tasks/236

fn main() {
    let s = read_line();
    let mut letters = std::collections::HashMap::new();
    for c in s.chars() {
        *letters.entry(c).or_insert(0) += 1;
    }
    println!("{}", letters.len());
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
