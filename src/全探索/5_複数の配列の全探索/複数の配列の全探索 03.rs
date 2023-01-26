// https://algo-method.com/tasks/233

fn main() {
    read_line();
    let (a, b, c) = (read_nums(), read_nums(), read_nums());
    let count = a
        .iter()
        .flat_map(|&i| b.iter().map(move |j| (i, j)))
        .flat_map(|(i, j)| c.iter().filter(move |k| i + j == **k))
        .count();
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
