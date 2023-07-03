use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut v: VecDeque<String> = VecDeque::new();

    for _ in 0..n {
        input!{
            q: usize,
        }

        if q == 1 {
            input!{
                x: String,
            }
            v.push_back(x);
        } else if q == 2 {
            println!("{}", v.front().unwrap());
        } else if q == 3 {
            v.pop_front();
        }
    }
}
