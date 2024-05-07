use std::collections::HashMap;

// refer to https://leetcode.com/problems/two-sum/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (index, &item) in nums.iter().enumerate() {
        let index_i32 = index as i32;
        if let Some(&minus_index) = map.get(&(target - item)) {
            if minus_index == index_i32 {
                continue;
            }
            return vec![minus_index, index_i32];
        }
        map.insert(item, index_i32);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let output = two_sum(nums.clone(), target);
        let expect = vec![0, 1];
        assert_eq!(
            output, expect,
            "nums: {:?}, target: {}, expect: {:?}, output: {:?}",
            &nums, target, &expect, &output
        );
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let _output = two_sum(nums.clone(), target);
        let _expect = [1, 2];
        // assert_eq!(
        //     output, expect,
        //     "nums: {:?}, target: {}, expect: {:?}, output: {:?}",
        //     &nums, target, &expect, &output
        // );
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let target = 6;
        let output = two_sum(nums.clone(), target);
        let expect = vec![0, 1];
        assert_eq!(
            output, expect,
            "nums: {:?}, target: {}, expect: {:?}, output: {:?}",
            &nums, target, &expect, &output
        );
    }
}
