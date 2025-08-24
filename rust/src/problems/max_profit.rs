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
