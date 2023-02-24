use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    let mut x = vec![vec![0; w+2]; h+2];

    for i in 0..n {
        x[abcd[i].0][abcd[i].1] += 1;
        x[abcd[i].0][abcd[i].3+1] -= 1;
        x[abcd[i].2+1][abcd[i].1] -= 1;
        x[abcd[i].2+1][abcd[i].3+1] += 1;
    }

    let mut s = vec![vec![0; w+1]; h+1];

    for i in 1..=h {
        for j in 1..=w {
            s[i][j] = s[i][j-1] + x[i][j]; 
        }
    }

    for i in 1..=w {
        for j in 1..=h {
            s[j][i] += s[j-1][i];
        }
    }

    for i in 1..=h {
       let mut sep = "";
       for j in 1..=w {
            print!("{}{}", sep, s[i][j]);
            sep = " ";
       }
       print!("\n");
    }
}
