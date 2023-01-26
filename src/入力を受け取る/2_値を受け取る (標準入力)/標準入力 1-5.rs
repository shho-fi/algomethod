// https://algo-method.com/tasks/23

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let x: i32 = line.trim().parse().unwrap();
    println!("{}", 24 - x);
}
