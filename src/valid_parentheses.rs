use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let bracket_map = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    let mut stack: Vec<char> = Vec::new();
    for item in s.chars() {
        if let Some(open_bracket) = bracket_map.get(&item) {
            if let Some(last) = stack.last() {
                if open_bracket != last {
                  return false;
                }
                stack.pop();
                continue;
            }
            return false;
        } else {
            stack.push(item);
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn example_2() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn example_3() {
        assert!(!is_valid("(]".to_string()));
    }
}
