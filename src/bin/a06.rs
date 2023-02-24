use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }

    let mut s = vec![0; n+1];

    s[0] = a[0];
    for i in 1..=n {
       s[i] = s[i-1] + a[i-1]; 
    }

    for i in 0..q {
        println!("{}", s[lr[i].1]-s[lr[i].0-1]);
    }
}
