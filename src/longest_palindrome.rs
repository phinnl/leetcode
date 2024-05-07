
// refer to https://leetcode.com/problems/longest-palindrome/

pub fn longest_palindrome(s: String) -> String {
    let equal = |option: Option<&char>, item: char| -> bool {
        match option {
            Some(x) => *x == item,
            None => false,
        }
    };
    let mut palindrome = String::from("");
    let s_bytes = s
        .as_bytes()
        .iter()
        .map(|&item| item as char)
        .collect::<Vec<char>>();
      let length = s.len();
    for (index, item) in s.chars().enumerate() {
        let mut max_temp = item.to_string();
        let max_index = if index > (length / 2) {
            length - 1 - index
        } else {
            index
        };
        let op_prev_item = if index > 0 {
            s_bytes.get(index - 1)
        } else {
            None
        };
        if equal(op_prev_item, item) {
            max_temp = format!("{}{}", op_prev_item.unwrap(), op_prev_item.unwrap());
            if max_temp.len() > palindrome.len() {
                palindrome.clone_from(&max_temp);
            }
            for i in 1..max_index+1 {
              if (index as i32 - 1 - i as i32) < 0 || index + i >= length {
                break;
              } 
              let item_prev_some = s_bytes.get(index - 1 - i);
              let item_next_some = s_bytes.get(index + i);
              if item_prev_some.is_none() || item_next_some.is_none() {
                break;
              }
              let item_prev = *s_bytes.get(index - 1 - i).unwrap();
              let item_next = *s_bytes.get(index + i).unwrap();
                if item_prev != item_next {
                    break;
                }
                max_temp = format!("{item_prev}{max_temp}{item_next}");

                if max_temp.len() > palindrome.len() {
                    palindrome.clone_from(&max_temp);
                }
            }
            max_temp = item.to_string();
        }
        if max_index == 0 {
            if max_temp.len() > palindrome.len() {
                palindrome.clone_from(&max_temp);
            }
            continue;
        }
        for i in 0..max_index {
            let item_prev_some = s_bytes.get(index - 1 - i);
            let item_next_some = s_bytes.get(index + 1 + i);
            if item_prev_some.is_none() || item_next_some.is_none() {
                break;
            }
            let item_prev = *item_prev_some.unwrap();
            let item_next = *item_next_some.unwrap();
            if item_prev != item_next {
                break;
            }
            max_temp = format!("{item_prev}{max_temp}{item_next}");

            if max_temp.len() > palindrome.len() {
                palindrome.clone_from(&max_temp);
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
        assert_eq!(longest_palindrome("bb".to_owned()), "bb".to_owned());
    }

    #[test]
    fn example_3() {
        assert_eq!(
          longest_palindrome("xaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa".to_owned()), "aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa".to_owned()
        
        );
    }
}
