use proconio::{fastout, input};
use rand::Rng;
//use std::cmp;

fn get_distance(xy1:(f64, f64), xy2:(f64, f64)) -> f64 {
    return (((xy2.0 - xy1.0).powf(2.0) + (xy2.1 - xy1.1).powf(2.0))).sqrt();
}

fn get_score(p: &Vec<usize>, xy: &Vec<(f64, f64)>) -> f64{
    let mut score: f64 = 0.0;
    for i in 0..p.len()-1 {
        score += get_distance(xy[p[i]], xy[p[i+1]]);
    }
    return score;
}

fn randouble() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen::<f64>();
}

#[fastout]
fn main() {
    input!{
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut p: Vec<usize> = (0..=n).collect();
    p[n] = 0;

    let mut current_score = get_score(&p, &xy);

    let mut rng = rand::thread_rng();
    for i in 0..200000 {
        let l = rng.gen_range(1, n);
        let r = rng.gen_range(1, n);

        if l == r { continue; }

        p.swap(l, r);
        let new_score = get_score(&p, &xy);

        // 
        let t: f64 = 30.0 - 28.0 * i as f64 / 200000.0;
        let mut z = (current_score - new_score) / t;
        if 0.0 < z { z = 0.0; }
        let probability: f64 = z.exp();

        //if current_score > new_score { 
        if randouble() < probability {
            current_score = new_score;
        } else {
            p.swap(l, r);
        }
    }

    for i in 0..=n {
        println!("{}", p[i]+1);
    }
}
