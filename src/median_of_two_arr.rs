// refer to https://leetcode.com/problems/median-of-two-sorted-arrays/

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
