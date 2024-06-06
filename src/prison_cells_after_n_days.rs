pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
    let mut cells = cells;
    let mut start = (n - 1) % 14 + 1;
    while start > 0 {
        start -= 1;
        let mut next = vec![0; 8];
        for i in 1..7 {
            next[i] = if cells[i - 1] == cells[i + 1] { 1 } else { 0 };
        }
        cells = next;
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
            vec![0, 0, 1, 1, 0, 0, 0, 0]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
            vec![0, 0, 1, 1, 1, 1, 1, 0]
        );
    }
}
