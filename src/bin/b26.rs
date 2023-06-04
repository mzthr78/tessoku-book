use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut is_prime = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if !is_prime[i] { continue; }

        println!("{}", i);

        for j in ((i*2)..=n).step_by(i) {
            is_prime[j] = false;   
        }
    }
}
