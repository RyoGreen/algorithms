use proconio::input;

pub fn shift_only() {
    input! {
        n:i32,
        mut nums:[i32;n],
    }
    let mut count: i32 = 0;
    loop {
        for num in &mut nums {
            if *num % 2 == 0 {
                *num /= 2;
            } else {
                println!("{}", count);
                return;
            }
        }
        count += 1;
    }
}
