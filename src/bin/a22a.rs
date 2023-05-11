use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        mut a: [usize; n-1],
        mut b: [usize; n-1],
    }

    a.insert(0, 0);
    b.insert(0, 0);
    
    let mut s = vec![std::isize::MIN; n+1];

    s[1] = 0;

    for i in 1..n {
        s[a[i]] = max(100+s[i], s[a[i]]);
        s[b[i]] = max(150+s[i], s[b[i]]);
    }

    println!("{}", s[n]);
}
