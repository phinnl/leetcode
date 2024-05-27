// refer to https://leetcode.com/problems/hamming-distance/

pub fn hamming_distance(x: i32, y: i32) -> i32 {
  let mut a = x ^ y;
  let mut count = 0;

  while a > 0 {
      count += a & 1;
      a >>= 1;
  }

  count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(hamming_distance(1, 4), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(hamming_distance(3, 1), 1);
    }
}