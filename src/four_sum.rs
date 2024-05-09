// refer to https://leetcode.com/problems/4sum/

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if nums.len() < 4 {
        return result;
    }

    nums.sort();

    for i in 0..nums.len() - 3 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in i + 1..nums.len() - 2 {
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }
            let mut left = j + 1;
            let mut right = nums.len() - 1;
            while left < right {
                if let Some(sum) = nums[i]
                    .checked_add(nums[j])
                    .and_then(|s| s.checked_add(nums[left]))
                    .and_then(|s| s.checked_add(nums[right]))
                {
                    match sum.cmp(&target) {
                        std::cmp::Ordering::Equal => {
                            result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            left += 1;
                            while left < right && nums[left] == nums[left - 1] {
                                left += 1;
                            }
                        }
                        std::cmp::Ordering::Greater => {
                            right -= 1;
                        }
                        std::cmp::Ordering::Less => {
                            left += 1;
                        }
                    }
                } else {
                    left += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        assert_eq!(four_sum(nums, target), expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![2, 2, 2, 2, 2];
        let target = 8;
        let expected = vec![vec![2, 2, 2, 2]];
        assert_eq!(four_sum(nums, target), expected);
    }
}
