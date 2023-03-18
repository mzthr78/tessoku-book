use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut xor_sum = a[0];

    for i in 1..n {
        xor_sum ^= a[i];
    }

    if xor_sum != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
