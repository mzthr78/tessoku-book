use proconio::input;

fn binary_search(v: &Vec<usize>, k: usize) -> isize {
    let mut l = 0;
    let mut r = v.len() - 1;

    let mut a: isize = -1;

    while l <= r {
        let m = (l + r) / 2;    
        
        if v[m] > k { r = m - 1; }
        else if v[m] < k { l = m + 1; }
        else { 
            a = m as isize; 
            break;
        }
    }

    return a;
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = a.clone();
    b.sort();

    let mut c = Vec::new();
    c.push(b[0]);
    for i in 1..n {
       if b[i] != b[i-1] { c.push(b[i]); } 
    }

    let mut x = vec!(0; n);
    for i in 0..n {
        x[i] = binary_search(&c, a[i]) + 1;
    }

    /*
    for i in 0..n {
        print!("{} ", x[i]);
    }
    print!("\n");
    */

    let s = x
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", s);
}
