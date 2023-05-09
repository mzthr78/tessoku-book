use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        w: isize,
        l: isize,
        r: isize,
        mut x: [isize; n],
    }

    x.push(w);
    x.push(0);
    x.rotate_right(1);

    let mut dp: Vec<isize> = vec![0; n+2];
    let mut sum: Vec<isize> = vec![0; n+2];

    let div = 1_000_000_007;

    dp[0] = 1;
    sum[0] = 1;

    for i in 1..=n+1 {
        let pl = x.binary_search(&(x[i]-r)).unwrap_or_else(|x| x);
        let pr = x.binary_search(&(x[i]-l+1)).unwrap_or_else(|x| x) as isize - 1;

        if pr == -1 { dp[i] = 0; }
        else { dp[i] = sum[pr as usize]; }

        if pl >= 1 { dp[i] -= sum[pl-1]; }
        dp[i] = (dp[i] + div) % div;

        sum[i] = sum[i-1] + dp[i];
        sum[i] %= div;
    }

    println!("{}", dp[n+1]);
}
