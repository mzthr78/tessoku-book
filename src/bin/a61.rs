use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut v = vec![vec![]; n+1];

    for (a, b) in ab {
        v[a].push(b);
        v[b].push(a);
    }

    for i in 1..v.len() {
        //v[i].sort();
        println!("{}: {{{}}}", i, 
            v[i]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
        );
    }
}
