use proconio::{fastout, input};

fn remainder(a: usize, b: usize, x: usize) -> usize {
    let mut p = a;
    let mut ans = 1;

    for i in 0..30 {
        if (b / (1<<i)) % 2 == 1 {
            ans = (ans * p) % x;
        }
        p = (p * p) % x;
    }
    return ans;
}

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", remainder(a, b, 1000000007));
}
