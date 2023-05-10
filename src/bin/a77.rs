use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }

    let mut left = 1;
    let mut right = l;

    while left < right {
        let mid = (left + right + 1) / 2;

        let mut count = 0;
        let mut last = 0;

        for i in 0..n {
            if a[i] - last >= mid && l - a[i] >= mid {
                count += 1; 
                last = a[i];
            }
        }

        if count >= k {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    println!("{}", left);
}
