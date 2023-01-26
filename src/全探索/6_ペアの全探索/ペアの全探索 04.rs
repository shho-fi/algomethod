// https://algo-method.com/tasks/260

use std::collections::HashSet;

fn main() {
    let n = read_nums()[0] as usize;
    let set: HashSet<String> = (0..n).map(|_| read_line()).collect();
    println!("{}", to_yes_or_no(set.len() < n));
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

fn to_yes_or_no(boolean: bool) -> String {
    (if boolean { "Yes" } else { "No" }).to_string()
}
