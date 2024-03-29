use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        mut a: usize,
        b: usize,
    }

    let mut ans = 1;

    for i in 0..=30 {
        if (b & (1<<i)) == 1<<i {
            ans = ans * a % 1_000_000_007;
        }
        a = a * a % 1_000_000_007;
    }

    println!("{}", ans);
}
