// https://algo-method.com/tasks/1029

fn main() {
    input! {
        n: usize,
        l: [usize; n],
        q: usize,
        ab: [(usize, usize); q]
    }
    let max_length = 100000;
    let strings = l.iter().fold(vec![0; max_length], |mut acc, l| {
        acc[*l] += 1;
        acc
    });
    let cumsum = (1..=max_length).fold(vec![0; max_length + 1], |mut acc, i| {
        acc[i] = acc[i - 1] + strings[i];
        acc
    });
    for (a, b) in ab {
        println!("{}", cumsum[b] - cumsum[a - 1]);
    }
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
