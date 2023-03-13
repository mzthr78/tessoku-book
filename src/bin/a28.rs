use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        ta: [(char, isize); n],
    }

    let x = 10000;
    let mut answer = 0;

    for &(t, a) in ta.iter() {
        match t {
            '+' => answer += a,
            '-' => answer -= a,
            '*' => answer *= a,
            _ => continue,
        }
        
        if answer < 0 { answer += x };
        answer %= x;
        
        println!("{}", answer);
    }
}
