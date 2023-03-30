use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        _n: usize,
        c: char,
        a: String,
    }

    let mut score = 0;
    for b in a.chars() {
        match b {
            'W' => score += 0,
            'B' => score += 1,
            'R' => score += 2,
            _ => continue,
        }
    }

    let mut ans = "No";

    if score % 3 == 0 && c == 'W'
    || score % 3 == 1 && c == 'B'
    || score % 3 == 2 && c == 'R'
        { ans = "Yes"; }

    println!("{}", ans);
}
