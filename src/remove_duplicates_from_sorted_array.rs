// refer to https://leetcode.com/problems/remove-duplicates-from-sorted-array/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  if nums.len() < 2 {
      return nums.len() as i32;
  }
  let mut i = 1;
  while i < nums.len() {
      if nums[i] == nums[i - 1] {
          nums.remove(i);
          continue;
      }
      i += 1;
  }
  nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}