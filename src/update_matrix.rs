pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut matrix = mat;
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut vec = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 0 {
                vec.push((i, j));
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] != 0 {
                let mut min = i32::MAX;
                for &(x, y) in vec.iter() {
                    let cur_distance = (i as i32 - x as i32).abs() + (j as i32 - y as i32).abs();
                    if i == 2 && j == 1 {
                      println!("{x} {y} {:?}", cur_distance);
                    }
                    if cur_distance < min {
                        min = cur_distance;
                    }
                }
                matrix[i][j] = min;
            }
        }
    }

    matrix
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }
}