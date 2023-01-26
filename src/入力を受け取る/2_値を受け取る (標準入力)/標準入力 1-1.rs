// https://algo-method.com/tasks/15

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    let n: i32 = line.trim().parse().unwrap();
    println!("{}", n * 2);
}
