use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut ans = vec![0; 4];

    for _ in 0..n {
        input!{
            a: isize,
            b: isize,
        }

        if a + b > 0 { ans[0] += a + b; }
        if a - b > 0 { ans[1] += a - b; }
        if -a + b > 0 { ans[2] += b - a; }
        if -a - b > 0 { ans[3] += -a - b; }
    }

    println!("{}", ans.iter().max().unwrap());
}
