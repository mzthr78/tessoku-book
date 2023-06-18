use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort_by_key(|lr| lr.1);

    let mut ans = 0;
    let mut pr = 0;

    for (l, r) in lr {
        if l < pr { continue; }
        ans += 1;
        pr = r;
    }

    println!("{}", ans);
}
