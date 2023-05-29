use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![];
    let mut x = 0;

    for i in 0..n {
        let p = match dp.binary_search(&a[i]) {
            Ok(v) => v,
            Err(v) => v
        };


        if p == x {
            dp.push(a[i]);
            x += 1;
        } else {
            dp[p] = a[i];
        }
        //println!("[{}] p={}, dp={:?}", i, p, dp);
    }

    //println!("a={:?}", a);
    //println!("dp={:?}", dp);
    //println!("answer={}", x);
    println!("{}", x);
}

