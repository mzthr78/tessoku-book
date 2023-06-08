use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    println!("{}", n/3+n/5+n/7-n/(3*5)-n/(5*7)-n/(3*7)+n/(3*5*7));
}
