use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.insert(0, 0);

    let mut ans = 0;

    for i in 1..=n {
        if a[i] > k { continue; }
        ans += 1;

        let mut sum = a[i];
        for j in i+1..=n {
            sum += a[j];
            if sum > k { break; }
            ans += 1;
        }
    }

    println!("{}", ans);
}
