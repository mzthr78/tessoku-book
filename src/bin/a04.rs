use proconio::input;

fn main() {
    input! {
        mut n: i32,
    }

    for i in 0..10 {
        print!("{}", n / 2_i32.pow(9-i));
        n = n - (n / 2_i32.pow(9-i) * 2_i32.pow(9-i));
    }
    print!("\n");
}
