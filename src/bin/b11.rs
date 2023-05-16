use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    }

    a.sort();

    for i in 0..q {
        println!("{}", binary_search(&a, x[i]));
    }
}

fn binary_search(a: &Vec<usize>, x: usize) -> usize {
    let mut l = 0;
    let mut r = a.len();

    while l < r {
        let m = (l + r) / 2;

        if a[m] < x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    return l;
}
