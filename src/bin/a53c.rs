use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input!{
        n: usize,
    }

    let mut item = BinaryHeap::new();

    for _i in 0..n {
        input!{
            q: usize,
        }
        if q == 1 {
            input!{
                price: usize,
            }
            item.push(Reverse(price));
        } else if q == 2 {
            println!("{}", item.peek().unwrap().0);
        } else if q == 3 {
            item.pop();
        }
    }
}
