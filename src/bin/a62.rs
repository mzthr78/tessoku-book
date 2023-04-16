use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    
    let mut g = vec![vec![]; n+1];

    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut v = vec![false; n+1];
    v[0] = true;

    dfs(&g, 1, &mut v);

    //println!("{:?}", v);

    if v.contains(&false) {
        println!("The graph is not connected.");
    } else {
        println!("The graph is connected.");
    }
}

fn dfs(g: &Vec<Vec<usize>>, x: usize, v: &mut Vec<bool>) {
    v[x] = true;

    for i in 0..g[x].len() {
        if !v[g[x][i]] {
            dfs(g, g[x][i], v);
        }
    }
}
