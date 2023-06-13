use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut x = a[0];

    for i in 1..n {
        x ^= a[i];
    }

    if x == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
