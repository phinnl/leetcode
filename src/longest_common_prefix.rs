pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::new();
    let first_str_bytes = strs[0].as_bytes();
    'outer: for index in 0..first_str_bytes.len() {
        let item = first_str_bytes[index] as char;
        for str_next in strs[1..].iter() {
            if let Some(item_next) = str_next.as_bytes().get(index) {
                if item == *item_next as char {
                    continue;
                }
            }
            break 'outer;
        }
        result.push(item);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(strs), "fl");
    }

    #[test]
    fn example_2() {
        let strs = vec!["ab".to_string(), "a".to_string()];
        assert_eq!(longest_common_prefix(strs), "a");
    }
}
