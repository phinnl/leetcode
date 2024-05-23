// refer to https://leetcode.com/problems/long-pressed-name/

pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    if name.len() > typed.len() {
        return false;
    }
    let mut idx = 0;
    let name = name.as_bytes();
    let typed = typed.as_bytes();

    for i in 0..typed.len() {
        if idx >= name.len() {
            if typed[i] == typed[i - 1] {
                continue;
            }
            return false;
        }
        if typed[i] == name[idx] {
            idx += 1;
        } else if idx > 0 && typed[i] == typed[i - 1] {
            continue;
        } else {
            return false;
        }
    }
    idx == name.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let name = "alex".to_string();
        let typed = "aaleex".to_string();
        assert!(is_long_pressed_name(name, typed));
    }

    #[test]
    fn example_2() {
        let name = "saeed".to_string();
        let typed = "ssaaedd".to_string();
        assert!(!is_long_pressed_name(name, typed));
    }
}
