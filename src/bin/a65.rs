use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n-1],
    }

    let mut b = vec![0; n+1];

    for i in (2..=n).rev() {
        b[a[i-2]] += b[i] + 1;
    }

    println!("{}", b[1..=n]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
    );
}
