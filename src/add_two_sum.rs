pub mod add_two_sum_mod {
    use std::collections::HashMap;

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map: HashMap<i32, i32> = nums
            .iter()
            .enumerate()
            .map(|(index, &item)| (item, i32::try_from(index).unwrap()))
            .collect();
        println!("{:?}", map);
        let mut vec: Option<Vec<i32>> = None;
        for (index, &item) in nums[..(nums.len() - 1)].iter().enumerate() {
            let index_i32 = i32::try_from(index).unwrap();
            if let Some(&minus_index) = map.get(&(target - item)) {
                if minus_index == index_i32 {
                    continue;
                }
                vec = Some(vec![index_i32, minus_index]);
                break;
            }
        }
        vec.unwrap()
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
            let output = two_sum(nums.clone(), target);
            let expect = vec![1, 2];
            assert_eq!(
                output, expect,
                "nums: {:?}, target: {}, expect: {:?}, output: {:?}",
                &nums, target, &expect, &output
            );
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
}
