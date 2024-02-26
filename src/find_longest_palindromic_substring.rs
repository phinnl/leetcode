pub mod longest_palindromic {

    pub fn longest_palindrome(s: String) -> String {
        let equal = |option: Option<char>, item: char| -> bool {
            match option {
                Some(x) => x == item,
                None => false,
            }
        };
        let mut palindrome = String::from("");
        for (index, item) in s.chars().enumerate() {
            let mut max_temp = item.to_string();
            let max_index = if index > (s.len() / 2) {
                s.len() - 1 - index
            } else {
                index
            };
            if index > 0 && equal(s.chars().nth(index - 1), item) {
                max_temp = format!(
                    "{}{}",
                    s.chars().nth(index - 1).unwrap(),
                    s.chars().nth(index - 1).unwrap()
                );
                if max_temp.len() > palindrome.len() {
                    palindrome = max_temp.clone();
                }
                for i in 1..max_index {
                    let item_prev = s.chars().nth(index - 1 - i).unwrap();
                    let item_next = s.chars().nth(index + i).unwrap();
                    if item_prev != item_next {
                        break;
                    }
                    max_temp = format!("{item_prev}{max_temp}{item_next}");

                    if max_temp.len() > palindrome.len() {
                        palindrome = max_temp.clone();
                    }
                }
                max_temp = item.to_string();
            }
            if max_index == 0 {
                if max_temp.len() > palindrome.len() {
                    palindrome = max_temp.clone();
                }
                continue;
            }
            for i in 0..max_index {
                let item_prev_some = s.chars().nth(index - 1 - i);
                let item_next_some = s.chars().nth(index + 1 + i);
                if item_prev_some.is_none() || item_next_some.is_none() {
                    break;
                }
                let item_prev = item_prev_some.unwrap();
                let item_next = item_next_some.unwrap();
                if item_prev != item_next {
                    break;
                }
                max_temp = format!("{item_prev}{max_temp}{item_next}");

                if max_temp.len() > palindrome.len() {
                    palindrome = max_temp.clone();
                }
            }
        }
        palindrome
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn example_1() {
            assert_eq!(longest_palindrome("babad".to_owned()), "bab".to_owned());
        }

        #[test]
        fn example_2() {
            assert_eq!(longest_palindrome("cbbd".to_owned()), "bb".to_owned());
        }

        #[test]
        fn example_3() {
            assert_eq!(longest_palindrome("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_owned()), "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_owned());
        }
    }
}
