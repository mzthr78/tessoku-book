use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut a: [usize; n-1],
        mut b: [usize; n-2],
    }

    for _ in 0..2 { a.push(0); } a.rotate_right(2);
    for _ in 0..3 { b.push(0); } b.rotate_right(3);

    let mut d: Vec<(usize, usize)> = vec![(0, 0); n+1];

    d[2] = (a[2], 1);

    for i in 3..=n {
        if d[i-1].0+a[i] < d[i-2].0+b[i] {
            d[i] = (d[i-1].0+a[i], i-1);
        } else {
            d[i] = (d[i-2].0+b[i], i-2);
        }
    }

    let mut p: Vec<usize> = vec![];
    let mut i = n;

    while i > 0 {
        p.push(i);
        i = d[i].1;
    }

    println!("{}", p.len());
    let mut s = "";
    for i in p.iter().rev() {
        print!("{}{}", s, i);
        s = " ";
    }
    print!("\n");
}
