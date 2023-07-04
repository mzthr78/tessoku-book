use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut hm: HashMap<usize, usize> = HashMap::new();
    let mut ans = 0;

    for ai in a {
        if hm.contains_key(&ai) {
            ans += hm[&ai];
        }
        *hm.entry(ai).or_insert(0) += 1;
    }

    println!("{}", ans);
}
