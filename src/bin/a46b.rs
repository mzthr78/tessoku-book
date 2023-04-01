use proconio::{fastout, input};
use rand::Rng;

fn get_distance(xy1:(isize, isize), xy2:(isize, isize)) -> isize {
    return (((xy2.0 - xy1.0).pow(2) + (xy2.1 - xy1.1).pow(2)) as f64).sqrt() as isize;
}

fn get_score(p: &Vec<usize>, xy: &Vec<(isize, isize)>) -> isize {
    let mut score: isize = 0;
    for i in 0..p.len()-1 {
        score += get_distance(xy[p[i]], xy[p[i+1]]);
    }
    return score;
}

#[fastout]
fn main() {
    input!{
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut p: Vec<usize> = (0..=n).collect();
    p[n] = 0;

    let mut current_score = get_score(&p, &xy);

    let mut rng = rand::thread_rng();
    for _i in 0..200000 {
        let l = rng.gen_range(1, n);
        let r = rng.gen_range(1, n);

        if l == r { continue; }

        p.swap(l, r);
        let new_score = get_score(&p, &xy);

        if current_score > new_score { 
            current_score = new_score;
        } else {
            p.swap(l, r);
        }
    }

    for i in 0..=n {
        println!("{}", p[i]+1);
    }
}
