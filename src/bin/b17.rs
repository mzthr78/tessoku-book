use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut h: [isize; n],
    }
    h.insert(0, 0);

    let mut dp = vec![0; n+1];
    let mut p = vec![0; n+1];

    dp[2] = (h[1] - h[2]).abs();
    p[2] = 1;

    for i in 3..=n {
        let c2 = (h[i-2]-h[i]).abs() + dp[i-2];
        let c1 = (h[i-1]-h[i]).abs() + dp[i-1];

        if c2 <= c1 {
            dp[i] = c2;
            p[i] = i - 2;
        } else {
            dp[i] = c1;
            p[i] = i - 1;
        }
    }

    let mut ans = vec![n; 1];

    let mut i = n;
    while i > 1 {
        ans.push(p[i]);
        i = p[i];
    }

    println!("{}", ans.len());
    let mut s = "";
    while !ans.is_empty() {
        print!("{}{}", s, ans.pop().unwrap());
        s = " ";
    }
    print!("\n");
}
