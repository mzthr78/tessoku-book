use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut a: Vec<isize> = vec![0; t+1];

    for (l, r) in lr {
        a[l] += 1;
        a[r] -= 1;
    }

    let mut ans = 0;
    for i in 0..t {
        ans += a[i];
        println!("{}", ans);
    }
}
