use rust::problems::max_profit::Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

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
