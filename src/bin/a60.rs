use proconio::{fastout ,input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut level2: Vec<(usize, usize)> = Vec::new();
    let mut ans: Vec<isize> = Vec::new();

    for i in 0..n {
        if i >=1 {
            level2.push((i, a[i-1]));
            while !level2.is_empty() {
                let kabuka = level2[level2.len()-1].1;
                if kabuka <= a[i] {
                    level2.pop();
                } else {
                    break;
                }
            }
        }

        if !level2.is_empty() {
            ans.push(level2[level2.len()-1].0 as isize);
        } else {
            ans.push(-1);
        }
    }

    let mut sep = "";

    for i in 0..n {
        print!("{}{}", sep, ans[i]);
        sep = " ";
    }
    print!("\n");
}
