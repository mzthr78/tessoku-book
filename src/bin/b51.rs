use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!{
        s: Chars,
    }

    let mut v: Vec<usize> = Vec::new();

    for i in 0..s.len() {
        if s[i] == '(' {
            v.push(i+1);
        } else {
            println!("{} {}", v.pop().unwrap(), i+1);
        }
    }
}
