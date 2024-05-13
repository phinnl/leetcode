// refer to https://leetcode.com/problems/add-strings/

pub fn add_strings(num1: String, num2: String) -> String {
    let len = num1.len().max(num2.len());
    let mut plus = 0;
    let mut result = String::new();
    for i in 0..len {
        let num1_index = (num1.len() as isize) - 1 - i as isize;
        let num2_index = (num2.len() as isize) - 1 - i as isize;
        let (a, b) = (
            num1.chars()
                .nth(num1_index as usize)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap(),
            num2.chars()
                .nth(num2_index as usize)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap(),
        );
        let sum = a + b + plus;
        if sum >= 10 {
            plus = sum / 10;
            result = format!("{}{}", sum - plus * 10, result);
        } else {
            result = format!("{}{}", sum, result);
            plus = 0;
        }
        if i == len - 1 && plus > 0 {
            result = format!("{}{}", plus, result);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            add_strings("11".to_string(), "123".to_string()),
            "134".to_string()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            add_strings("456".to_string(), "77".to_string()),
            "533".to_string()
        );
    }
}
