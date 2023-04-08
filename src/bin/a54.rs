use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut grades = HashMap::new();

    for _i in 0..n {
        input!{
            q: usize,
        }

        if q == 1 {
            input!{
                name: String,
                score: usize,
            }
            grades.insert(name, score);
        } else if q == 2 {
            input!{
                name: String,
            }
            println!("{}", grades.get(&name).unwrap()); 
        }
    }
}
