// refer to https://leetcode.com/problems/flipping-an-image/

pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    image
        .into_iter()
        .map(|mut arr| {
            let len = arr.len();
            let half_len = (len + 1) / 2;
            for i in 0..half_len {
                let (left, right) = (arr[i], arr[len - 1 - i]);
                arr[i] = if right == 0 { 1 } else { 0 };
                arr[len - 1 - i] = if left == 0 { 1 } else { 0 };
            }
            arr
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ]),
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ]
        )
    }
}
