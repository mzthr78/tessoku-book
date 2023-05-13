use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    //let mut sum = vec![(0, 0); n+1];
    let mut sum = vec![0; n+1];

    for i in 0..n {
        /*
        sum[i+1] = sum[i];
        if a[i] == 0 { sum[i+1].0 += 1; }
        if a[i] == 1 { sum[i+1].1 += 1; }
        */
        sum[i+1] = sum[i] + a[i];
    }

    //println!("{:?}", sum);

    for (l, r) in lr {
        /*
        let win = sum[r].1 - sum[l-1].1;
        let lose = sum[r].0 - sum[l-1].0;
        */
        let win = sum[r] - sum[l-1];
        let lose =  r - l + 1 - win;

        if win > lose { println!("win"); }
        else if win == lose { println!("draw"); }
        else { println!("lose"); }
    }

}
