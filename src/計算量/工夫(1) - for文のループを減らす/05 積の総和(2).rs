// https://algo-method.com/tasks/1021

fn main() {
    read_line();
    let a = read_nums();
    let (sum_a, sum_doubles) = a.iter().fold((0, 0), |(sum_a, sum_doubles), x| {
        (sum_a + x, sum_doubles + x * x)
    });
    println!("{}", (sum_a * sum_a - sum_doubles) / 2);
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
