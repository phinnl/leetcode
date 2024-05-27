use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut map: HashMap<char, &str> = HashMap::new();
    let s_by_words: Vec<&str> = s.split(' ').collect();

    if pattern.len() != s_by_words.len() {
        return false;
    }

    for i in 0..pattern.len() {
        if map.contains_key(&pattern.chars().nth(i).unwrap()) {
            if map.get(&pattern.chars().nth(i).unwrap()) != Some(&s_by_words[i]) {
                return false;
            }
        } else {
            if map.values().any(|&x| x == s_by_words[i]) {
                return false;
            }
            map.insert(pattern.chars().nth(i).unwrap(), s_by_words[i]);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(word_pattern(
            "abba".to_string(),
            "dog cat cat dog".to_string()
        ));
    }

    #[test]
    fn example_2() {
        assert!(!word_pattern(
            "abba".to_string(),
            "dog cat cat fish".to_string()
        ));
    }

    #[test]
    fn example_3() {
        assert!(!word_pattern(
            "aaaa".to_string(),
            "dog cat cat dog".to_string()
        ));
    }
}
