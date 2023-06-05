use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        ta: [(char, isize); n],
    }

    let mut ans: isize = 0;

    for (t, a) in ta {
        if t == '+' {
            ans += a;
        } else if t == '-' {
            ans -= a;
        } else if t == '*' {
            ans *= a;
        }
        if ans < 0 { ans += 10000; }
        ans %= 10000;

        println!("{}", ans);
    }
}
