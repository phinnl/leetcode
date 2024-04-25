pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    while left < right {
        let area = (right - left) as i32 * height[left].min(height[right]);
        if area > max {
            max = area;
        }
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let output = 49;
        assert_eq!(max_area(height), output);
    }

    #[test]
    fn example_2() {
        let height = vec![1, 1];
        let output = 1;
        assert_eq!(max_area(height), output);
    }

    #[test]
    fn example_3() {
        let height = vec![2, 3, 4, 5, 18, 17, 6];
        let output = 17;
        assert_eq!(max_area(height), output);
    }
}
