use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.insert(0, 0);

    let mut s = vec![0; n+1];

    for i in 1..=n {
        s[i] = s[i-1] + a[i];
    }

    let mut p = vec![0; n+1];

    for i in 1..=n {
        p[i] = p[i-1];

        for j in p[i]..=n {
            if s[j] > k + s[i-1] { break; }
            p[i] = j;
        }
    }

    let mut ans = 0;

    for i in 1..=n {
        if p[i] < i { continue; }
        ans += p[i] - i + 1;
    }

    println!("{}", ans);
}
