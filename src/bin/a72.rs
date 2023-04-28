use proconio::{fastout, input, marker::Chars};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        h: usize,
        w: usize,
        k: usize,
        mut c: [Chars; h],
    }

    let mut ans = 0;

    for b in 0..1<<h {
        let mut d = c.clone();
        let mut r: isize = k as isize;

        for i in 0..h {
            if (b / (1<<i)) % 2 == 0 { continue; }
            r -= 1;
            d[i] = (0..w).map(|_| '#').collect::<Vec<char>>();
        }

        if r >= 0 {
            let mut col: Vec<(usize, usize)> = Vec::new();

            for j in 0..w {
                let mut cnt = 0;
                for i in 0..h {
                    if d[i][j] == '.' { cnt += 1; }
                }
                col.push((cnt, j));
            }
            col.sort();
            col.reverse();

            for j in 0..(r as usize) {
                for i in 0..h {
                    d[i][col[j].1] = '#';
                }
            }

            let mut s = 0;
            for i in 0..h {
                for j in 0..w {
                    if d[i][j] == '#' { s += 1; }
                }
            }

            ans = max(ans, s);
        }
    }

    println!("{}", ans);
}
