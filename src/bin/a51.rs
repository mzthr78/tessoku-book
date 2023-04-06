use proconio::{fastout, input};

//#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut desk = vec![];

    for _i in 0..n {
        input!{
            q: usize,
        }

        match q {
            1 => {
                input!{
                    title: String
                }

                desk.push(title);
            },
            2 => {
                println!("{}", desk[desk.len()-1]);
            },
            3 => {
                desk.pop();
            },
            _ => continue,
        }
    }
}
