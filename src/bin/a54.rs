use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut hm = HashMap::new();

    for _ in 0..n {
        input!{
            q: usize,
        }

        if q == 1 {
            input!{
                name: String,
                score: usize,
            }
            hm.insert(name, score);
        } else if q == 2 {
            input!{
                name: String,
            }
            println!("{}", hm[&name]);
        }
    }
}
