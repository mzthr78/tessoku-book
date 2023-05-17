use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.insert(0, 0);

    let mut p = vec![0; n];

    p[0] = 1;

    for i in 1..n {
        p[i] = p[i-1];

        for j in p[i]..=n {
            if a[j] - a[i] <= k {
                p[i] = j;
            }
        }
    }

    let mut ans = 0;
    for i in 1..n {
        ans += p[i] - i;
    }

    println!("{}", ans);
}
