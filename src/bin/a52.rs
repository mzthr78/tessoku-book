use proconio::{fastout, input};

//#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut queue = vec![];

    for _i in 0..n {
        input!{
            q: usize,
        }

        match q {
            1 => {
                input!{
                    x: String,
                }
                queue.push(x);
            },
            2 => println!("{}", queue[0]),
            3 => {
                let _s = queue.remove(0);
            },
            _ => continue,
        }
    }
}
