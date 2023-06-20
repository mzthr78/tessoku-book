use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut m = vec![0; 100];

    for i in 0..n {
        m[a[i] % 100] += 1;
    }

    let mut ans = 0;

    ans += (m[0] * (m[0]-1)) / (2 * 1);
    ans += (m[50] * (m[50]-1)) / (2 * 1);

    for i in 1..50 {
        ans += m[i] * m[100-i];
    }

    println!("{}", ans);
}
