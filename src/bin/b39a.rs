use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        d: usize,
        xy: [(usize, usize); n],
    }

    let mut z = vec![false; n];
    let mut ans = 0;

    for i in 1..=d {
        let mut mi: isize = -1;
        for j in 0..n {
            if z[j] { continue; }

            if xy[j].0 <= i && (mi == -1 || xy[j].1 > xy[mi as usize].1) {
                mi = j as isize;
            }
        }

        if mi > -1 {
            ans += xy[mi as usize].1;
            z[mi as usize] = true;
        }
    }

    println!("{}", ans);
}
