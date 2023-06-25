//use proconio::{fastout, input};
use proconio::input;

//#[fastout]
fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let mut a: Vec<usize> = (0..=n).collect();

    let mut asc = true;

    for _ in 0..q {
        input!{
            query: usize,
        }

        if query == 1 {
            input!{
                mut x: usize,
                y: usize,
            }
            if !asc { x = n + 1 - x; }
            a[x] = y;
        } else if query == 2 {
            asc = !asc;
        } else if query == 3 {
            input!{
                mut x: usize,
            }
            if !asc { x = n + 1 - x; }
            println!("{}", a[x]);
        }
    }
}
