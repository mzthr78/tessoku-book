use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        qxy: [(usize, usize, usize); m],
    }

    let mut t: Vec<usize> = (0..n).collect();

    for (q, x, y) in qxy {
        if q == 1 {
           t.swap(x-1, y-1); 
        } else if q == 2 {
           println!("{}", a[t[x-1]][y-1]);
        }
    }
}
