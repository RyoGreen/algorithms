pub struct Solution;
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut prev = 0;
        let mut next = 1;

        for _ in 2..=n {
            (prev, next) = (next, prev + next)
        }

        next
    }
}
