use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!{
        _n: usize,
        k: usize,
        s: Chars,
    }

    let mut m = 0;

    for c in s {
        if c == '1' { m += 1; }
    }

    if m % 2 == k % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
