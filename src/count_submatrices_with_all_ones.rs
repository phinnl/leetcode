// refer to https://leetcode.com/problems/count-submatrices-with-all-ones/

pub fn num_submat(mut matrix: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let len = matrix.len();
    for i in 0..len {
        for j in 0..matrix[0].len() {
            if j == 0 || matrix[i][j] == 0 {
                continue;
            }
            if let Some(item) = matrix
                .get(i)
                .and_then(|item| item.get(j - 1))
                .and_then(|&item| if item == 0 { None } else { Some(item) })
            {
                matrix[i][j] += item;
            }
        }
    }
    for i in 0..len {
        for j in 0..matrix[0].len() {
            let mut k = i;
            let mut l = i32::MAX;
            loop {
                l = matrix[k][j].min(l);
                if l == 0 {
                    break;
                }
                result += l;
                if k == 0 {
                    break;
                }
                k -= 1;
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
        let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(num_submat(matrix), 13);
    }

    #[test]
    fn example_2() {
        let matrix = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]];
        assert_eq!(num_submat(matrix), 24);
    }
}
