use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let nl = n / 2;
    let nr = n - (n / 2);

    let mut sl: Vec<usize> = vec![];
    let mut sr: Vec<usize> = vec![];

    for i in 1..(1<<nl) {
        let mut sum = 0;
        for j in 0..nl {
            if i & (1<<j) == (1<<j) {
                sum += a[j];
            }
        }
        if sum == k {
            println!("Yes");
            return;
        }
        sl.push(sum);
    }

    for i in 1..(1<<nr) {
        let mut sum = 0;
        for j in 0..nr {
            if i & (1<<j) == (1<<j) {
                sum += a[j+nl];
            }
        }
        if sum == k {
            println!("Yes");
            return;
        }
        sr.push(sum);
    }

    sr.sort();

    for i in 0..(1<<nl)-1 {
        let mut l = 0;
        let mut r = sr.len() - 1;

        while l < r {
            let m = (l + r) / 2;

            if sl[i] + sr[m] == k {
                println!("Yes");
                return;
            } else if sl[i] + sr[m] > k {
                r = m;
            } else {
                l = m + 1;
            }
        }
    }
    println!("No");
}
