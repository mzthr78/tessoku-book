use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        q: usize,
        s: String,
        lr: [(usize, usize); q],
    }

    let t = &s.chars().rev().collect::<String>();

    for (l, r) in lr {
        if &s[l-1..r] == &t[n-r..=n-l] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
