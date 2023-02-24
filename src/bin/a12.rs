use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut l = 1;
    let mut r = 10_usize.pow(9);

    while l < r {
        let m = (l + r) / 2;

        //println!("l={}, r={}, m={}", l, r, m);

        let mut p = 0;
        for i in 0..n {
            p += m / a[i];
        }
        
        if p < k { l = m + 1; }
        else { r = m; }
    }

    println!("{}", l);
}
