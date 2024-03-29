use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        mut a: usize,
        mut b: usize,
    }

    while a > 0 && b > 0 {
        if a >= b {
            a %= b;
        } else {
            b %= a;
        }
    }

    if a != 0 {
        println!("{}", a);
    } else {
        println!("{}", b);
    }
}
