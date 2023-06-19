use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        mut x: usize,
        mut y: usize,
    }

    let mut xy: Vec<(usize, usize)> = Vec::new();

    while x > 1 || y > 1 {
        xy.push((x, y));

        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }

    xy.reverse();

    println!("{}", xy.len());

    for (x, y) in xy {
        println!("{} {}", x, y);
    }
}
