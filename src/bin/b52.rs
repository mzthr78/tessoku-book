use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize,
        x: usize,
        mut a: Chars,
    }
    a.insert(0, ' ');

    let mut q = VecDeque::new();
    q.push_back(x);
    a[x] = '@';

    while !q.is_empty() {
        let p = q.pop_front().unwrap();

        if p > 1 && a[p-1] == '.' {
            a[p-1] = '@';
            q.push_back(p-1);
        }

        if p < n && a[p+1] == '.' {
            a[p+1] = '@';
            q.push_back(p+1);
        }
    }

    for i in 1..=n {
        print!("{}", a[i]);
    }
    print!("\n");
}
