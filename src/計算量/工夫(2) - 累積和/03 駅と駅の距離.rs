// https://algo-method.com/tasks/1027

fn main() {
    let n = scan!(usize);
    let dists = scan!([usize])
        .iter()
        .enumerate()
        .fold(vec![0; n + 1], |mut acc, (i, x)| {
            acc[i + 1] = acc[i] + *x;
            acc
        });
    for _ in 0..scan!(usize) {
        let line = scan!([usize]);
        let (l, r) = (line[0], line[1]);
        println!("{}", dists[r] - dists[l]);
    }
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
