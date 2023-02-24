use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut count = 0;
    for i in 0..n-1 {
        let mut l = i + 1;
        let mut r = n;
        while l < r {
            let m = (l + r) / 2;
            if a[m] - a[i] > k {
                r = m; 
            } else {
                l = m + 1;
            }
        }
        l -= 1;
        count += l - i;
        //println!("i={}, l={}, a[{}]={}, a[{}]={}",i, l, i, a[i], l, a[l]);
    }
    println!("{}", count);
}
