// https://algo-method.com/tasks/232

fn main() {
    let nmk = read_nums();
    let (n, m, k) = (nmk[0], nmk[1], nmk[2]);
    let a = read_nums();
    let b = read_nums();
    let mut count = 0;
    for x in a {
        count += b.iter().filter(|y| x + **y == k).count();
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
