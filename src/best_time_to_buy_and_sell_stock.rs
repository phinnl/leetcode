// refer to https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

pub fn max_profit(prices: Vec<i32>) -> i32 {
  prices.iter()
  .skip(1)
  .fold((0, prices[0]), |(diff, min), &n| match n <= min {
      true => (diff, n),
      false => (diff.max(n - min), min),
  })
  .0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(max_profit(prices), 5);
  }

  #[test]
  fn example_2() {
    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(max_profit(prices), 0);
  }
}