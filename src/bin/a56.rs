use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        _n: usize,
        q: usize,
        s: String,
        abcd: [(usize, usize, usize, usize); q],
    }

    for &(a, b, c, d) in &abcd {
        if s[a-1..b] == s[c-1..d] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
