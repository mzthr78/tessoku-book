use proconio::{fastout, input, marker::Chars};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: Chars,
    }

    let mut g1 = vec![1; n];
    let mut g2 = vec![1; n];

    for i in 0..n-1 {
        if s[i] == 'A' {
            g1[i+1] = g1[i] + 1;
        } else {
            g1[i+1] = 1;
        }
    }

    for i in (0..n-1).rev() {
        if s[i] == 'B' { 
            g2[i] = g2[i+1] + 1;
        } else {
            g2[i] = 1;
        }
    }

    let mut ans = 0;

    for i in 0..n {
        ans += max(g1[i], g2[i]);
    }

    println!("{}", ans);
}

