// https://algo-method.com/tasks/225

fn main() {
    let n = read_nums()[0];
    for i in 1..=n {
        println!(
            "{}",
            match (i % 3, i % 5) {
                (0, 0) => "FizzBuzz",
                (0, _) => "Fizz",
                (_, 0) => "Buzz",
                _ => {
                    print! {"{}", i};
                    ""
                }
            }
        )
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
