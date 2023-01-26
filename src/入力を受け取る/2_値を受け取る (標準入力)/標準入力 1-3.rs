// https://algo-method.com/tasks/21

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    let s: String = line.trim().parse().unwrap();
    println!("{}", s.repeat(3));
}
