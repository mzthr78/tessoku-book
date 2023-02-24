use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    let mut s = vec![vec![0; w+1]; h+1];


    for i in 1..=h {
        for j in 1..=w { 
            s[i][j] = s[i][j-1] + x[i-1][j-1];
        }
    }

    for j in 1..=w {
        for i in 1..=h {
            s[i][j] = s[i][j] + s[i-1][j];
        }
    }

    for i in 0..q {
        let answer = s[abcd[i].2][abcd[i].3] - s[abcd[i].0-1][abcd[i].3] - s[abcd[i].2][abcd[i].1-1] + s[abcd[i].0-1][abcd[i].1-1];
        println!("{}", answer);
    }
}
