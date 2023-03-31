use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        mut xy: [(i64, i64); n],
    }

    let mut ans: Vec<usize> = vec![];
    let mut ab: (i64, i64) = (0, 0);
    let mut idx: Vec<usize> = (0..n).collect();
    
    for _i in 0..n-1 {
        let mut min_i = 150;
        let mut min_v = 1001*1001;
        for j in 0..idx.len() {
            let tmp = (xy[idx[j]].0-ab.0).abs()+(xy[idx[j]].1-ab.1).abs();
            if tmp < min_v {
                min_v = tmp;
                min_i = j;
            }
        }
        ab = xy[idx[min_i]];
        ans.push(idx[min_i]);
        idx.remove(min_i);
    }
    ans.push(idx[0]);
    ans.push(ans[0]);

    for i in 0..ans.len() {
        println!("{}", ans[i]+1);
    }
}
