use proconio::{fastout, input};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut bh = BinaryHeap::new();

    for _ in 0..n {
        input!{
            q: usize,
        }

        if q == 1 {
            input!{
                x: usize,
            }
            bh.push(Reverse(x));
        } else if q == 2 {
            println!("{}", bh.peek().unwrap().0);
        } else if q == 3 {
            bh.pop();
        }
    }
}
