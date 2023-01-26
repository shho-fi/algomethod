// https://algo-method.com/tasks/217

fn main() {
    read_line();
    let mut answers = [0; 9];
    read_nums()
        .into_iter()
        .for_each(|a| answers[a as usize - 1] += 1);
    answers.iter().for_each(|i| println!("{}", i));
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
