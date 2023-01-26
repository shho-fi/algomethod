// https://algo-method.com/tasks/62

use std::collections::HashMap;

fn main() {
    let n = read_nums()[0];
    let mut counts = HashMap::new();
    (0..n).for_each(|_| *counts.entry(read_line()).or_insert(0) += 1);
    let count_right = counts.get("right");
    let count_left = counts.get("left");
    println!(
        "{}",
        if count_right > count_left {
            "right"
        } else if count_right == count_left {
            "same"
        } else {
            "left"
        }
    );
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
