use proconio::{fastout, input};
use std::cmp;

struct SegTree {
    dat: Vec<usize>,
    siz: usize,
}

impl SegTree {
    fn new(n: usize) -> SegTree {
        let mut tmp = 1;
        while tmp < n {
            tmp *= 2;
        }

        SegTree {
            dat: vec![0; tmp*2],
            siz: tmp,
        }
    }

    fn update(&mut self, mut pos: usize, x: usize) {
        pos = pos + self.siz - 1;
        self.dat[pos] = x;
        while pos >= 2 {
            pos /= 2;
            self.dat[pos] = cmp::max(self.dat[pos*2], self.dat[pos*2+1]);
        }
    }

    fn query(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> isize {
        if r <= a || b <= l { return -1000000000; }
        if l <= a && b <= r { return self.dat[u] as isize; }
        let m = (a + b) / 2;
        let ans_l = self.query(l, r, a, m, u * 2);
        let ans_r = self.query(l, r, m, b, u * 2 + 1);
        return cmp::max(ans_l, ans_r);
    }
}

#[fastout]
fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let mut z: SegTree = SegTree::new(n);

    for _ in 0..q {
        input!{
           query: usize,
        }

        if query == 1 {
            input!{
                pos: usize,
                x: usize,
            }
            z.update(pos, x);
        } else if query == 2 {
            input!{
                l: usize,
                r: usize,
            }
            println!("{}", z.query(l, r, 1, z.siz+1, 1));
        }
    }
}

