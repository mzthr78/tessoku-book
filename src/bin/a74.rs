use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        p: [[usize; n]; n],
    }

    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            if p[i][j] > 0 {
                x[j] = p[i][j];
                y[i] = p[i][j];
            }
        }
    }

    let mut ansx = 0;
    let mut ansy = 0;

    for i in 0..n-1 {
        for j in i+1..n {
            if x[i] > x[j] { ansx += 1; }
            if y[i] > y[j] { ansy += 1; }
        }
    }

    println!("{}", ansx + ansy);
}
