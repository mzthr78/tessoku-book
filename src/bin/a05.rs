use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let mut answer: i32 = 0;
    for i in 1..=n {
        for j in 1..=n {
            let t: i32 = k - i - j;
            if t <= n && t >= 1 { answer += 1 }
        }
    }
    println!("{}", answer);
}
