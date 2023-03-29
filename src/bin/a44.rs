//use proconio::{fastout, input};
use proconio::input;

//#[fastout]
fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let mut a: Vec<usize> = (1..=n).collect();
    let mut dir = true;

    for _ in 0..q {
        input!{
            q0: usize,
        }

        if q0 == 1 {
            input!{
                q1: usize,
                q2: usize,
            }
            if dir {
                a[q1-1] = q2;
            } else {
                a[n-q1] = q2;
            }
        } else if q0 == 2 { 
            dir = !dir;
        } else if q0 == 3 {
            input!{
                q1: usize,
            }

            if dir {
                println!("{}", a[q1-1]);
            } else {
                println!("{}", a[n-q1]);
            }
        }
    }
}
