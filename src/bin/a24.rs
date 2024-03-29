use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut dp: Vec<usize> = vec![];

    for i in 0..n {
        let p = binary_search(&dp, a[i]);

        if p == dp.len() {
            dp.push(a[i]);
        } else {
            dp[p] = a[i];
        }
    }

    println!("{}", dp.len());
}

fn binary_search(a: &Vec<usize>, x: usize) -> usize {
    let mut l = 0;
    let mut r = a.len();

    while l < r {
        let m = (l + r) / 2;

        if a[m] < x {
            l = m + 1;
        } else {
            r = m;
        }
    }

    return l;
}
