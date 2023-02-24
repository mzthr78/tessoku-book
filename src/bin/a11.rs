use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut l = 0;
    let mut r = n - 1;
    let mut m = (l + r) / 2;

    //println!("l={}, r={}, m={}, a[{}]={}", l, r, m, m, a[m]);

    loop {
        if a[m] > x {
            r = m - 1;
        } else if a[m] < x {
            l = m + 1;
        } else {
            println!("{}", m+1);
            break;
        }
        m = (l + r) / 2;

        //println!("l={}, r={}, m={}, a[{}]={}", l, r, m, m, a[m]);
    }
}
