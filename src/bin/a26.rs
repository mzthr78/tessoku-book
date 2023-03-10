use proconio::input;
use proconio::fastout;

fn is_prime(x: usize) -> bool {
    let mut i = 2;
    while i*i <= x {
        if x % i == 0 { return false; }
        i += 1;
    }
    return true;
}

#[fastout]
fn main() {
    input!{
        q: usize,
        x: [usize; q],
    }

    for i in 0..q {
        if is_prime(x[i]) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
