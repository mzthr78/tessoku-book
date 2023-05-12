use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        mut n: usize,
    }

    let mut sum = 0;
    let mut i = 1;
    while n > 0 {
        sum += (n & 1) * i;
        n /= 10;
        i = i << 1;
    }
    println!("{}", sum);
}
