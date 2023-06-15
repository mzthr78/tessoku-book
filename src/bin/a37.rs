use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        b: usize,
        a: [usize; n],
        c: [usize; m],
    }

    let sum_a: usize = a.iter().sum();
    let sum_c: usize = c.iter().sum();

    println!("{}", sum_a * m + b * m * n + sum_c * n);
}
