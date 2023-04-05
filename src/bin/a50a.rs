use proconio::{fastout, input};
use rand::Rng;

#[fastout]
fn main() {
    //let n = 5;
    let n = 100;

    input!{
        _a: [[usize; n]; n],
    }

    let mut xyh = vec![(0, 0, 1); 1000];

    for i in 0..1000 {
        let mut rng = rand::thread_rng();
        //xyh[i] = (rng.gen_range(0..100), rng.gen_range(0..100), 1);
        xyh[i] = (rng.gen_range(0, 100), rng.gen_range(0, 100), 1);
    }

    println!("{}", 1000);
    for i in 0..1000 {
        let (x, y, h) = xyh[i];
        println!("{} {} {}", x, y, h);
    }
}
