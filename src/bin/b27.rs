use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        mut a: usize,
        mut b: usize,
    }

    let c = a * b;

    while a > 0 && b > 0 {
        if a >= b {
            a %= b;
        } else {
            b %= a;
        }
    }

    //println!("{}", c / (if a != 0 { a } else { b }));
    println!("{}", c / (a | b));
}
