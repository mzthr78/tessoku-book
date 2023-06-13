use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }

    let mut gr = vec![0; 100000+1];

    for i in 0..=100000 {
        let mut tr = vec![false; 3];

        if i >= x { tr[gr[i-x]] = true; }
        if i >= y { tr[gr[i-y]] = true; }

        if !tr[0] { gr[i] = 0; }
        else if !tr[1] { gr[i] = 1; }
        else { gr[i] = 2; }
    }

    let mut s = 0;

    for i in 0..n {
        s ^= gr[a[i]];
    }

    if s != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
