// refer to https://leetcode.com/problems/string-to-integer-atoi/

pub fn my_atoi(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut temp = String::new();
    for (index, &i_byte) in bytes.iter().enumerate() {
        let item = i_byte as char;
        let next_item = bytes.get(index + 1);
        if item.is_numeric()
            || ((item == '-' || item == '+')
                && next_item.is_some()
                && (*next_item.unwrap() as char).is_numeric()
                && temp.is_empty())
        {
            temp = format!("{temp}{item}");
        } else if item == ' ' {
            continue;
        } else {
            break;
        }
        if next_item.is_none() || !(*next_item.unwrap() as char).is_numeric() {
            break;
        }
    }

    match temp.parse::<i32>() {
        Ok(i) => i,
        Err(_) => {
            if temp.starts_with('-') {
                i32::MIN
            } else if temp.chars().any(|c| c.is_numeric()) {
                i32::MAX
            } else {
                0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "words and 987".to_owned();
        assert_eq!(0, my_atoi(s));
    }
}
