// https://algo-method.com/tasks/224

fn main() {
    let line = read_nums();
    let (a, b) = (line[0], line[1]);
    println!("{}", gcd(a, b));
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
