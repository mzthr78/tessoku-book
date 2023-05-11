use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        mut a: [usize; n-1],
        mut b: [usize; n-2],
    }

    for _ in 0..2 { a.push(0); } a.rotate_right(2);
    for _ in 0..3 { b.push(0); } b.rotate_right(3);

    let mut d = vec![0; n+1];

    d[2] = a[2];

    for i in 3..=n {
        d[i] = min(d[i-1]+a[i], d[i-2]+b[i]);
    }

    println!("{}", d[n]);
}
