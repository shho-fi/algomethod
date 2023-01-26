// https://algo-method.com/tasks/259

fn main() {
    let n = read_nums()[0] as usize;
    let a = read_nums();
    let count = (0..n)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .flat_map(|(i, j)| (0..n).map(move |k| (i, j, k)))
        .filter(|(i, j, k)| i < j && j < k && &a[*j] == [a[*i], a[*j], a[*k]].iter().max().unwrap())
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
