use proconio::{input, fastout};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input!{
        n: usize,
        qx: [(usize, usize); n],
    }

    let mut cards = BTreeSet::new();

    for &(q, x) in &qx {
        if q == 1 {
            cards.insert(x);
        } else if q == 2 {
            cards.remove(&x);
        } else if q == 3 {
            /*
            match cards.range(x..).next() {
                Some(y) => println!("{}", y),
                None => println!("{}", -1),
            }
            */
            if let Some(m) = cards.range(x..).next() {
                println!("{}", m);
            } else {
                println!("{}", -1);
            }
        }
    }
}
