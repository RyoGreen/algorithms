use proconio::input;

fn main() {
    input! {
        a: String,
    }
    let mut sum: i8 = 0;
    for c in a.chars() {
        if c == '1' {
            sum += 1;
        }
    }
    println!("{}", sum);
}
