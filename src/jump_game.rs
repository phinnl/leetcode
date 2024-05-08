pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut last_idx = nums.len() - 1;

    for i in 1..nums.len() {
        let cur_index = nums.len() - 1 - i;
        if nums[cur_index] >= (last_idx - cur_index) as i32 {
            last_idx = cur_index;
        }
    }

    last_idx == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(can_jump(nums));
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!can_jump(nums));
    }
}
