use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    let mut x: Vec<Vec<isize>> = vec![vec![0; 1500+1]; 1500+1];

    for (a, b, c, d) in abcd {
        x[a][b] += 1;
        x[c][b] -= 1;
        x[c][d] += 1;
        x[a][d] -= 1;
    }

    for i in 0..=1500 {
        for j in 1..=1500 {
            x[i][j] += x[i][j-1];
        }
    }

    for j in 0..=1500 {
        for i in 1..=1500 {
            x[i][j] += x[i-1][j];
        }
    }

    let mut ans = 0;

    for i in 0..=1500 {
        for j in 0..=1500 {
            if x[i][j] > 0 { ans += 1; }
        }
    }

    println!("{}", ans);
}
