pub mod median_of_two_sorted_arr {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut nums1, mut nums2) = (nums1, nums2);
        nums1.append(&mut nums2);
        nums1.sort();
        let half_len = (nums1.len() as f64) / 2.0;
        if half_len.fract() == 0.0 {
            (nums1[half_len as usize] + nums1[half_len as usize - 1]) as f64 / 2.0
        } else {
            nums1[half_len as usize] as f64
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn example_1() {
            //       Input: nums1 = [1,3], nums2 = [2]
            // Output: 2.00000
            // Explanation: merged array = [1,2,3] and median is 2.
            // Example 2:

            // Input: nums1 = [1,2], nums2 = [3,4]
            let input_1 = vec![1, 3];
            let input_2 = vec![2];
            let expect: f64 = 2.0;
            let output = find_median_sorted_arrays(input_1.clone(), input_2.clone());
            assert_eq!(
                expect, output,
                "input 1: {:?}, input 2: {:?}, output: {output}",
                input_1, input_2
            );
        }
    }
}
