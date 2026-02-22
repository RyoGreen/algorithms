pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut result: i32 = 0;
        let mut buy_amount: i32 = prices[0];

        for &num in prices.iter() {
            if num < buy_amount {
                buy_amount = num;
            } else {
                let profit: i32 = num - buy_amount;
                if profit > result {
                    result = profit;
                }
            }
        }

        result
    }
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut result: i32 = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                result += prices[i] - prices[i - 1];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        // Test case 1: A typical case
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(prices), 5); // Buy at 1 and sell at 6

        // Test case 2: No profit case
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0); // No profit can be made

        // Test case 3: Single day price
        let prices = vec![3];
        assert_eq!(Solution::max_profit(prices), 0); // No profit with a single price

        // Test case 4: Empty list
        let prices: Vec<i32> = Vec::new();
        assert_eq!(Solution::max_profit(prices), 0); // No profit with no prices
    }
}
