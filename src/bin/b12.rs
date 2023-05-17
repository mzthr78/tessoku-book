use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: f64,
    }

    let mut x = 0.0;
    let mut l = 0.0;
    let mut r = 100.0;
    
    for _ in 0..27 {
        x = (l + r) / 2.0;
        let fx = x * x * x + x;

        if fx > n { r = x; }
        else { l = x; }
    }

    println!("{:.6}", x);
}
