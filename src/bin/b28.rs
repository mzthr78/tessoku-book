use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut a1 = 1;
    let mut a2 = 1;

    let mut ans = 0;
    for _ in 3..=n {
        ans = a2 + a1;
        ans %= 1000000007;

        a1 = a2;
        a2 = ans;
    }

    println!("{}", ans);
}
