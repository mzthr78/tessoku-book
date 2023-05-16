use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    }

    a.sort();

    for i in 0..q {
        println!("{}", a.binary_search(&x[i]).unwrap_or_else(|x| x));
    }
}
