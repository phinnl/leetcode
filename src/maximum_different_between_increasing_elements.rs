// refer to https://leetcode.com/problems/maximum-difference-between-increasing-elements/

pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    if length < 1 {
        return -1;
    }
    let max_option = nums
        .iter()
        .enumerate()
        .map(|(index, item)| {
            if index == length {
                return -1;
            }
            let next_index = index + 1;
            let child_max = nums[next_index..]
                .to_vec()
                .iter()
                .map(|next_item| {
                    if item >= next_item {
                        -1
                    } else {
                        next_item - item
                    }
                })
                .max();
            match child_max {
                Some(num) => num,
                _ => -1,
            }
        })
        .max();
    match max_option {
        Some(max) => max,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(maximum_difference(nums), 5);
    }
}
