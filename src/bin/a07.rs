use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut a = vec![0; d+1];

    for i in 0..n {
        a[lr[i].0-1] += 1;
        a[lr[i].1] -= 1;
    }

    let mut s = 0;
    for i in 0..d {
        s += a[i];
        println!("{}", s);
    }
}
