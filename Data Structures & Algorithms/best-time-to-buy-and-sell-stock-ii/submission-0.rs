impl Solution {
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut current_lowest = prices[0];
    let mut current_highest = prices[0];
    for price in prices.iter().skip(1) {
        if price > &current_highest {
            current_highest = *price;
        } else {
            profit += current_highest - current_lowest;
            current_lowest = *price;
            current_highest = *price;
        }
    }
    profit += current_highest - current_lowest;
    profit
}
}
