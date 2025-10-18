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
    pub fn fib_v1(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        return self::Solution::fib_v1(n - 1) + self::Solution::fib_v1(n - 2);
    }
}
