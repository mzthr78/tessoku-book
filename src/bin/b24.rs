use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut xy: [(usize, isize); n],
    }

    //xy.sort();
    xy.sort_by_key(|xy| (xy.0, -xy.1));

    let mut dp: Vec<(usize, isize)> = vec![];

    for i in 0..n {
        let p = binary_search(&dp, xy[i].1);

        if p == dp.len() {
            dp.push(xy[i]);
        } else {
            dp[p] = xy[i];
        }
    }

    println!("{}", dp.len());
}

fn binary_search(a: &Vec<(usize, isize)>, x: isize) -> usize {
    let mut l = 0;
    let mut r = a.len();

    while l < r {
        let m = (l + r) / 2;

        if a[m].1 < x {
            l = m + 1;
        } else {
            r = m;
        }
    }

    return l;
}
