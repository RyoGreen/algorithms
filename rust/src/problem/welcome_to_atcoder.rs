use proconio::input;

pub fn welcome_to_atcoder() {
    input! {
        a:i32,
        b:[i32;2],
        s:String,
    };
    println!("{} {}", a + b[0] + b[1], s)
}
