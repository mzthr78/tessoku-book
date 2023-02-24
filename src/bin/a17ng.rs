use proconio::input;
use proconio::fastout;
use std::cmp;

#[fastout]
fn main() {
    todo!();

    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    }

    let mut dp = vec![0; n];

    dp[0] = 0;
    dp[1] = a[0];

    for i in 2..n {
        dp[i] = cmp::min(dp[i-1]+a[i-1], dp[i-2]+b[i-2]);    
    }

    let mut answer = vec![];
    let mut count = n - 1;
    
    answer.push(count+1);

    loop {
        //if dp[count] - a[count-1] == dp[count - 1] {
        if dp[count] == dp[count-1] + a[count-1] {
            count -= 1;
        } else {
            count -= 2;
        }
        answer.push(count+1);
        if count <= 0 { break; }
    }

    println!("{}", answer.len());

    answer.reverse();
    println!("{}", answer
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" "));
}
