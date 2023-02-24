use proconio::input;
    
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut b = false;
    for i in 0..n {
        if a[i] == x {
            b = true;
            break;
        }
    }

    if b { println!("Yes"); }
    else { println!("No"); }
}
