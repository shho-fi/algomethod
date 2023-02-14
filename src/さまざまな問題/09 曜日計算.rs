// https://algo-method.com/tasks/1099

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        y0: i32,
        m0: i32,
        d0: i32,
        x: i32,
        y1: i32,
        m1: i32,
        d1: i32
    }
    let days0 = d0 - 1 + (m0 - 1) * b + (y0 - 1) * a * b;
    let days1 = d1 - 1 + (m1 - 1) * b + (y1 - 1) * a * b;
    let diff = (days1 - days0) % c;
    println!("{}", (x - 1 + diff + c) % c + 1);
}

#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}
#[macro_export]
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}
#[macro_export]
macro_rules! read_value {
    ($iter:expr, ($($t:tt),*)) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [$t:tt ; $len:expr]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}
