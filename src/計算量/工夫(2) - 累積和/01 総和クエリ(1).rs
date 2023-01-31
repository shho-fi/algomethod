// https://algo-method.com/tasks/1025

fn main() {
    let n = scan!(usize);
    let mut sums = vec![0; n + 1];
    for (i, a) in scan!([usize]).enumerate() {
        sums[i + 1] = sums[i] + a;
    }
    let q = scan!(usize);
    for _ in 0..q {
        println!("{}", sums[scan!(usize)]);
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
    };
    ($name:ty) => {
        scan!().parse::<$name>().unwrap()
    };
}
