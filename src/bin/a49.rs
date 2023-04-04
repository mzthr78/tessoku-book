use proconio::{fastout, input};
use std::cmp;

#[derive(Clone, Debug)]
struct State {
    score: isize,
    x: Vec<isize>,
    last_move: char,
    last_pos: usize,
}

#[fastout]
fn main() {
    input!{
        t: usize,
        pqr: [(usize, usize, usize); t],
    }

    let width = 10000;
    let n = 20;
    let mut num_state = vec![0; t+1];
    let mut beam = vec![vec![State{
        score: 0,
        x: vec![0; n+1],
        last_move: 'A',
        last_pos: 0,
    }; width]; t+1];
    let mut answer = vec![' '; t];
    
    //BeamSearch
    num_state[0] = 1;
    beam[0][0].score = 0;

    for i in 1..=t {
        let (p, q, r) = pqr[i-1];
        let mut candidate: Vec<State> = vec![];
        for j in 0..num_state[i-1] {
            let mut sousa_a = beam[i-1][j].clone();
            sousa_a.last_move = 'A';
            sousa_a.last_pos = j;
            sousa_a.x[p] += 1;
            sousa_a.x[q] += 1;
            sousa_a.x[r] += 1;
            for k in 1..=n {
                if sousa_a.x[k] == 0 { sousa_a.score += 1; } 
            }

            let mut sousa_b = beam[i-1][j].clone();
            sousa_b.last_move = 'B';
            sousa_b.last_pos = j;
            sousa_b.x[p] -= 1;
            sousa_b.x[q] -= 1;
            sousa_b.x[r] -= 1;
            for k in 1..=n {
                if sousa_b.x[k] == 0 { sousa_b.score += 1; }
            }

            candidate.push(sousa_a);
            candidate.push(sousa_b);
        }

        candidate.sort_by(|a, b| b.score.cmp(&a.score));
        num_state[i] = cmp::min(width, candidate.len());
        for j in 0..num_state[i-1] {
            beam[i][j] = candidate[j].clone();
        }
    }

    //
    let mut current = 0;
    for i in (1..=t).rev() {
        answer[i-1] = beam[i][current].last_move;
        current = beam[i][current].last_pos;
    }

    //output
    for i in 1..=t {
        println!("{}", answer[i-1]);
    }
}
