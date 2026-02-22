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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
        assert_eq!(Solution::fib(5), 5);
        assert_eq!(Solution::fib(6), 8);
    }

    #[test]
    fn test_fib_v1() {
        assert_eq!(Solution::fib_v1(2), 1);
        assert_eq!(Solution::fib_v1(3), 2);
        assert_eq!(Solution::fib_v1(4), 3);
        assert_eq!(Solution::fib_v1(5), 5);
        assert_eq!(Solution::fib_v1(6), 8);
    }
}
