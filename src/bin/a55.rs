use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input!{
        n: usize,
        qx: [(usize, isize); n],
    }

    let mut bs = BTreeSet::new();

    for (q, x) in qx {
        if q == 1 {
            bs.insert(x);
        } else if q == 2 {
            bs.remove(&x);
        } else if q == 3 {
            let ans = match bs.range(x..).next() {
                Some(y) => y,
                None => &-1,
            };
            println!("{}", ans);
        }
    }
}
