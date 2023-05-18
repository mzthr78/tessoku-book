use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut b = a.clone();
    b.sort();
    b.dedup();

    let mut c: Vec<usize> = vec![];
    for i in 0..n {
        c.push(b.binary_search(&a[i]).unwrap()+1);
    }

    let mut s = "";
    for i in 0..n {
        print!("{}{}", s, c[i]);
        s = " ";
    }
    print!("\n");
}
