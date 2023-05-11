use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        a: usize,
        b: usize,
    }

    let mut ans = "No";
    for i in a..=b {
        if 100 % i == 0 {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}
