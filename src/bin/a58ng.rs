use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let mut a = vec![0; n];

    for _ in 0..q {
        input!{
            b: usize,
        }

        if b == 1 {
            input!{
                pos: usize,
                x: isize,
            }
            a[pos-1] = x;
        } else if b == 2 {
            input!{
                l: usize,
                r: usize,
            }
            println!("{}", &a[l-1..r].iter().max().unwrap());
        }
    }
}
