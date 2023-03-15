use proconio::{fastout, input};

fn power_mod(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;
    for i in 0..30 {
        if (b / (1<<i)) % 2 == 1 {
            ans = (ans * p) % m;
        }
        p = (p * p) % m;
    }
    return ans;
}

#[fastout]
fn main() {
    input!{
        n: usize,
        r: usize,
    }

    let m = 1000000007;

    let mut a = 1;
    for i in 1..=n {
        a = (a * i) % m;
    }
    let mut b = 1;
    for i in 1..=r {
        b = (b * i) % m;
    }
    for i in 1..=(n-r) {
        b = (b * i) % m;
    }

    let ans = (a * power_mod(b, m - 2, m)) % m;

    println!("{}", ans);
}
