use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort_by(|a, b| a.1.cmp(&(b.1)));

    let mut now = 0;
    let mut ans = 0;
    for &(l, r) in &lr {
        if l < now { continue; }
        now = r;
        ans += 1;
    }

    println!("{}", ans);
}
