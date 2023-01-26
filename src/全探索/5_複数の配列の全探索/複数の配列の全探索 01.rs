// https://algo-method.com/tasks/231

fn main() {
    read_line();
    let (a, b) = (read_nums(), read_nums());
    let mut count = 0;
    for i in a {
        count += b.iter().filter(|j| i > **j).count();
    }
    println!("{}", count);
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
