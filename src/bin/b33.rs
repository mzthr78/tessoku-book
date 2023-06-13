use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        _h: usize,
        _w: usize,
        ab: [(usize, usize); n],
    }

    let mut x = 0;

    for (a, b) in ab {
        x ^= (a - 1) ^ (b - 1);
    }

    if x != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}

