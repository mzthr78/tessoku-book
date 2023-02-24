// A14
use proconio::input;
use proconio::fastout;

fn binary_search(v: &Vec<usize>, k: usize) -> isize {
  let mut l = 0;
  let mut r = v.len() - 1;

  while l <= r {
    let m = (l + r) / 2;

    //println!("v={:?} m={}", v, m);
    if v[m] > k && m > 0 { r = m - 1; }
    else if v[m] < k { l = m + 1; }
    else {
      return m as isize;
    }
  }
  return -1;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }
    
    let mut x1: Vec<usize> = Vec::new();
    let mut x2: Vec<usize> = Vec::new();
    
    for i in 0..n {
        for j in 0..n {
            x1.push(a[i] + b[j]);
            x2.push(c[i] + d[j]);
        }
    }

    x1.sort();
    x2.sort();

    //println!("{:?}", x1);
    //println!("{:?}", x2);

    let mut answer = "No";

    //println!("x1={:?}", x1);

    for i in 0..n*n {
        //println!("[{}] {}", i, x1[i]);
        if k >= x1[i] && binary_search(&x2, k-x1[i]) > 0 {
            answer = "Yes";
            break;
        }
    }

    println!("{}", answer);
}

