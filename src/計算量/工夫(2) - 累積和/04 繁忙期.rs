// https://algo-method.com/tasks/1028

fn main() {
    let line = scan!([usize]);
    let (n, d) = (line[0], line[1]);
    let start = scan!([i32])
        .iter()
        .enumerate()
        .fold(vec![0; n + 1], |mut acc, (i, x)| {
            acc[i + 1] = acc[i] + x;
            acc
        })
        .windows(d + 1)
        .enumerate()
        .max_by_key(|(_, w)| w[d] - w[0])
        .unwrap()
        .0;
    println!("{}~{}", start, start + d - 1);
}

#[macro_export]
macro_rules! scan {
    () => {{
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf);
        buf.trim().to_string()
    }};
    ([$name:ty]) => {
        scan!()
            .split_whitespace()
            .map(|s| s.parse::<$name>().unwrap())
            .collect::<Vec<$name>>()
    };
    ($name:ty) => {
        scan!().parse::<$name>().unwrap()
    };
}
