use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        h: usize,
        w: usize,
    }

    let x = 1_000_000_007;

    let n = h + w - 2;
    let r = w - 1;

    let mut a = 1;

    for i in 2..=n {
        a = a * i % x;
    }

    let mut b = 1;

    for i in 2..=r {
        b = b * i % x;
    }

    for i in 2..=(n-r) {
        b = b * i % x;
    }

    println!("{}", a * pwm(b, x - 2, x) % x);
}

fn pwm(mut a: usize, b: usize, c: usize) -> usize {
    let mut ans = 1;
    for i in 0..30 {
        if b & 1<<i == 1<<i {
            ans = ans * a % c;
        }
        a = a * a % c;
    }
    return ans;
}
