use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        a: [usize; n],
        xyz: [(usize, usize, usize); m],
    }

    let mut s = 0;
    for i in 0..n {
        s |= a[i]<<(n-i-1);
    }

    let mut b = vec![0; m];

    for i in 0..m {
        let (x, y, z) = xyz[i];

        b[i] |= 1<<(n-x);
        b[i] |= 1<<(n-y);
        b[i] |= 1<<(n-z);
    }

    let mut q: VecDeque<usize> = VecDeque::new();    
    let mut d = vec![0; 1<<n];

    q.push_back(s);
    d[s] = 1;

    while !q.is_empty() {
        let p = q.pop_front().unwrap();

        for i in 0..m {
            let n = p ^ b[i];

            if d[n] == 0 {
                d[n] = d[p] + 1;
                q.push_back(n);
            }
        }
    }

    println!("{}", (d[(1<<n)-1] as isize) - 1);
}

