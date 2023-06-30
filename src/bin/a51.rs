use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        q: usize,
    }

    let mut v: Vec<String> = Vec::new();

    for _ in 0..q {
        input!{
            x: usize,
        }

        if x == 1 {
            input!{
                b: String,
            }
            v.push(b);
        } else if x == 2 {
            println!("{}", v.last().unwrap());
        } else if x == 3 {
            v.pop();
        }
    }
}
