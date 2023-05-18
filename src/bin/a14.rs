use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }

    let mut ab = vec![0; n*n];
    let mut cd = vec![0; n*n];

    for i in 0..n {
        for j in 0..n {
            ab[i*n+j] = a[i] + b[j];
            cd[i*n+j] = c[i] + d[j];
        }
    }

    ab.sort();
    cd.sort();

    let mut ans = "No";

    for i in 0..n*n {
        let mut l = 0;
        let mut r = n*n-1;

        while l < r {
            let m = (l + r) / 2;

            if ab[i] + cd[m] == k {
                ans = "Yes";
                break;
            } else if ab[i] + cd[m] > k {
                r = m;
            } else {
                l = m + 1;
            }
        }
        if ans == "Yes" { break; }
    }

    println!("{}", ans);
}
