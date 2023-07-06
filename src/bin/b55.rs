use proconio::{fastout, input};
use std::collections::BTreeSet;
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        qx: [(isize, isize); n],
    }

    let mut bs1 = BTreeSet::new();

    for (q, x) in qx {
        if q == 1 {
            bs1.insert(x);
        } else if q == 2 {
            let v1 = match bs1.range(..x).last() {
                Some(y) => x - y,
                None => std::isize::MAX,
            };

            let v2 = match bs1.range(x..).next() {
                Some(y) => y - x,
                None => std::isize::MAX,
            };

            let ans = min(v1, v2);

            println!("{}", if ans == std::isize::MAX { -1 } else { ans });
        }
    }
}
