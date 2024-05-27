// refer to https://leetcode.com/problems/nim-game/

pub fn can_win_nim(n: i32) -> bool {
  if n < 4 {
      return true;
  }

  n % 4 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(!can_win_nim(4));
    }

    #[test]
    fn example_2() {
        assert!(can_win_nim(1));
    }
}