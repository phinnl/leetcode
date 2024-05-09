// refer to https://leetcode.com/problems/letter-combinations-of-a-phone-number/

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let mut result = Vec::<String>::new();
    let map = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let digits = digits
        .as_bytes()
        .iter()
        .map(|&item| {
            let index = (item as char).to_digit(10).unwrap() - 2;
            map[index as usize].to_owned()
        })
        .collect::<Vec<String>>();
    for item in digits {
        if result.is_empty() {
            result = item.chars().map(|c| c.to_string()).collect::<Vec<String>>();
            continue;
        }
        let mut temp_vec: Vec<String> = Vec::<String>::new();
        for cur in result {
            for char in item.chars() {
                temp_vec.push(format!("{cur}{char}"));
            }
        }
        result = temp_vec;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            letter_combinations("23".to_owned())
        );
    }
}
