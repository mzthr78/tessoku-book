use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        _n: usize,
        c: char,
        a: String,
    }

    let wbr = ['W', 'B', 'R'];
    /*
    let score = HashMap::from([
        ('W', 0),
        ('B', 1),
        ('R', 2),
    ]);
    */
    let mut score = HashMap::new();
    score.insert('W', 0);
    score.insert('B', 1);
    score.insert('R', 2);

    let mut ans = 0;

    for b in a.chars() {
        ans += score.get(&b).unwrap();
    }

    println!("{}", if wbr[ans % 3] == c { "Yes" } else { "No" });
}
