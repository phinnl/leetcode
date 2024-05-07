use std::cmp::max;

// refer to https://leetcode.com/problems/maximum-subarray/

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut result = i32::MIN;
    let mut sum = 0;
    for item in nums.iter() {
        sum += item;
        result = max(sum, result);
        sum = max(sum, 0);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
