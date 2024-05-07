use std::collections::HashSet;

pub fn repeated_character(s: String) -> char {
    let s_bytes = s.as_bytes();
    let mut result: Option<char> = None;
    'outer: for index in 1..s_bytes.len() {
        let mut left = index - 1;
        let cur: u8 = s_bytes[index];
        loop {
            if let Some(prev_item) = s_bytes.get(left) {
                if cur == *prev_item {
                    result = Some(cur as char);
                    break 'outer;
                }
            }
            if left == 0 {
              break;
            }
            left -= 1;
        }
    }
    result.unwrap()
}

pub fn repeated_character_2(s: String) -> char {
  let mut set: HashSet<u8> = HashSet::new();
  let mut result = ' ';
  for item in s.as_bytes() {
      if let Some(c) = set.get(item) {
          result = *c as char;
          break;
      }
      set.insert(*item);
  }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(repeated_character("abccbaacz".to_string()), 'c');
        assert_eq!(repeated_character_2("abccbaacz".to_string()), 'c');
    }

    #[test]
    fn example_2() {
        assert_eq!(repeated_character("abcdd".to_string()), 'd');
        assert_eq!(repeated_character_2("abcdd".to_string()), 'd');
    }
    
    #[test]
    fn example_3() {
        assert_eq!(repeated_character("revav".to_string()), 'v');
        assert_eq!(repeated_character_2("revav".to_string()), 'v');
    }
}
