use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        _x: usize,
        _y: usize,
        a: [usize; n],
    }

    let mut s = 0;

    for i in 0..n {
        if a[i] % 5 == 0 || a[i] % 5 == 1 { s ^= 0; }
        else if a[i] % 5 == 2 || a[i] % 5 == 3 { s ^= 1; }
        else { s ^= 2; }
    }

    if s != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
