// https://algo-method.com/tasks/219

fn main() {
    read_line();
    let mut counts = [0; 10];
    read_nums()
        .into_iter()
        .for_each(|a| counts[a as usize] += 1);
    println!(
        "{}",
        counts.iter().enumerate().max_by_key(|(_, &c)| c).unwrap().0
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
