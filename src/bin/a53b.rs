use proconio::input;

fn main() {
    input!{
        n: usize,
    }

    let mut item = vec![];

    for _i in 0..n {
        input!{
            q: usize,
        }
        if q == 1 {
            input!{
                price: usize,
            }
            item.push(price);
            //item.sort();
        } else if q == 2 {
            println!("{}", item.iter().min().unwrap());
        } else if q == 3 {
            let index = item.iter().position(|a| a==item.iter().min().unwrap()).unwrap();
            item.remove(index);
        } 
    }
}
