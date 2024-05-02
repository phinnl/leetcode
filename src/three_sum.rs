pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    nums.sort();
    let len = nums.len();
    for index in 0..len - 2 {
        if index > 0 && nums[index] == nums[index - 1] {
            continue;
        }
        let mut left = index + 1;
        let mut right = len - 1;
        while left < right {
            let sum = nums[index] + nums[left] + nums[right];
            match sum {
                0 => {
                    result.push(vec![nums[index], nums[left], nums[right]]);
                    left += 1;
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
                _ => {
                    if sum < 0 {
                        left += 1;
                    } else {
                        right -= 1;
                    }
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
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(three_sum(nums), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
