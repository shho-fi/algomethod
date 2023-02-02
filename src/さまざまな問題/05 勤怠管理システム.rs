// https://algo-method.com/tasks/1091

fn main() {
    input! {
        stamps: [(i32, i32, i32, i32); 30]
    }
    let sum: i32 = stamps
        .iter()
        .map(|(h1, m1, h2, m2)| {
            let mut duration = (h2 * 60 + m2) - (h1 * 60 + m1);
            duration -= if duration > 60 * 8 {
                60
            } else if duration > 60 * 6 {
                45
            } else {
                0
            };
            duration
        })
        .sum();
    println!("{} {}", sum / 60, sum % 60);
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
