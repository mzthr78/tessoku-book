use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut ans = vec![m; n+1];

    for i in 0..m {
        ans[a[i]] -= 1;
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
