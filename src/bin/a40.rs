use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut ai = vec![0; 100+1];

    for _ in 0..n {
        input!{
            a: usize,
        }
        ai[a] += 1;
    }

    let mut ans: isize = 0;

    for i in ai {
        ans += (i * (i-1) * (i-2)) / (3 * 2 * 1);
    }

    println!("{}", ans);
}
