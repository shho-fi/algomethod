// https://algo-method.com/tasks/16

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    let n: i32 = line.trim().parse().unwrap();
    println!("{}", n % 5);
}
