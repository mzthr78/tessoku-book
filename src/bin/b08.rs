use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    let mut p = vec![vec![0; 1500+1]; 1500+1];

    for (x, y) in xy {
        p[x][y] += 1;
    }

    let mut s = vec![vec![0; 1500+1]; 1500+1];

    for i in 1..=1500 {
        for j in 1..=1500 {
            s[i][j] = s[i][j-1] + p[i][j];
        }
    }

    for j in 1..=1500 {
        for i in 1..=1500 {
            s[i][j] += s[i-1][j];
        }
    }

    for (a, b, c, d) in abcd {
        println!("{}", s[c][d] - s[c][b-1] - s[a-1][d] + s[a-1][b-1]);
    }
}
