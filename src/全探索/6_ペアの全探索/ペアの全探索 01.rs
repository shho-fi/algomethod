// https://algo-method.com/tasks/244

fn main() {
    let line = read_nums();
    let (n, k) = (line[0], line[1]);
    let a = read_nums();
    let count = (0..n)
        .flat_map(|i| (1..n).filter(move |j| i < *j).map(move |j| (i, j)))
        .filter(|(i, j)| a[*i as usize] + a[*j as usize] <= k)
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
