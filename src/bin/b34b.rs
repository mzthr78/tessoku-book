use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        _x: usize,
        _y: usize,
        a: [usize; n],
    }

    let gr = vec![0, 0, 1, 1, 2];
    let mut x = 0;

    for i in 0..n {
        x ^= gr[a[i] % 5];
    }

    if x != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
