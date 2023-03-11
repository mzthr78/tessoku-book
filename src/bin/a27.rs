use proconio::{input, fastout};

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a >= 1 && b >= 1 {
        if a >= b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    if a != 0 { return a; }
    return b;
}

#[fastout]
fn main() {
    input!{
        a: usize,
        b: usize,
    }

    let answer = gcd(a, b);
    println!("{}", answer);
}
