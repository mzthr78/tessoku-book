use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut v = vec![false; n];
    let mut a: Vec<usize> = vec![];

    a.push(0);
    v[0] = true;
    let mut c = 0;

    for _i in 1..n {
        let mut min_i = 151;
        let mut min_v = 1001 * 1001;

        for j in 0..n {
            if v[j] { continue; }

            let mut d = 
                (xy[c].0 - xy[j].0)
              * (xy[c].0 - xy[j].0)
              - (xy[c].1 - xy[j].1) 
              * (xy[c].1 - xy[j].1)
            ;
            d = (d as f64).sqrt() as isize;

            if d < min_v {
                min_v = d;
                min_i = j;
            }
        }
        a.push(min_i);
        v[min_i] = true;
        c = min_i;
    }

    a.push(0);

    for &tmp in &a {
        println!("{}", tmp+1);
    }
}
