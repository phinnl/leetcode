// refer to https://leetcode.com/problems/height-checker/

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut height_swaps = heights.clone();
    let mut result = 0;
    for _ in 0..height_swaps.len() {
        for j in 0..height_swaps.len() - 1 {
            if height_swaps[j] > height_swaps[j + 1] {
                height_swaps.swap(j, j + 1);
            }
        }
    }
    for i in 0..height_swaps.len() {
        if heights[i] != height_swaps[i] {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(height_checker(vec![5, 1, 2, 3, 4]), 5);
    }

    #[test]
    fn example_3() {
        assert_eq!(height_checker(vec![1, 2, 3, 4, 5]), 0);
    }
}
