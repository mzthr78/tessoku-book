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
    
    let mut s = vec![-1; n+1];
    s[1] = 0;

    for i in 1..n {
        if s[i] >= 0 {
            s[a[i]] = max(s[i]+100, s[a[i]]);
            s[b[i]] = max(s[i]+150, s[b[i]]);
        } 
    }

    println!("{}", s[n]);
}
