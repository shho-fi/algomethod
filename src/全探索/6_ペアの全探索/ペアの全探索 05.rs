// https://algo-method.com/tasks/261

use std::collections::HashMap;

fn main() {
    read_line();
    let map = read_line().chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    println!(
        "{}",
        map.iter()
            .map(|(_, count)| count_combination(*count))
            .sum::<i32>()
    );
}

fn count_combination(num: i32) -> i32 {
    num * (num - 1) / 2
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
