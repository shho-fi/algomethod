// https://algo-method.com/tasks/235

fn main() {
    let line = read_nums();
    let (n, k) = (line[0], line[1]);
    println!("{}", (1..=n).filter(|&x| count_divisors(x) == k).count());
}

fn count_divisors(num: i64) -> i64 {
    let num_sqrt = (num as f64).sqrt() as i64;
    (1..=num_sqrt)
        .filter(|&x| num % x == 0)
        .map(|x| if num / x > num_sqrt { 2 } else { 1 })
        .sum::<i64>()
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
