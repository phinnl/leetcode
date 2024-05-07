// refer to https://leetcode.com/problems/3sum-closest/

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut result: Option<i32> = None;
    nums.sort();
    'outer: for index in 0..nums.len() - 2 {
        let mut left = index + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[index] + nums[left] + nums[right];
            if result.is_none() {
              result = Some(sum);
              continue;
            }
            let data_result = result.unwrap(); 
            if sum == target {
                result = Some(target);
                break 'outer;
            }
            let closest = if sum > target {
                right -= 1;
                (sum - target) as u32
            } else {
                left += 1;
                (target - sum) as u32
            };
            let old_closest = if data_result > target {
                (data_result - target) as u32
            } else {
                (target - data_result) as u32
            };
            if closest < old_closest {
                result = Some(sum);
            }
        }
    }
    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(three_sum_closest(vec![1, 1, -1, -1, 3], -1), -1);
    }

    #[test]
    fn example_3() {
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn example_4() {
        assert_eq!(three_sum_closest(vec![1, 1, 1, 0], 100), 3);
    }
    
    #[test]
    fn example_5() {
        assert_eq!(three_sum_closest(vec![1, 1, 1, 0], -100), 2);
    }
}
