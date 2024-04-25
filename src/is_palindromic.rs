pub fn is_palindrome(x: i32) -> bool {
  // 1. not fast -> x.to_string() == x.to_string().chars().rev().collect::<String>()
  // 2. fast
  let st = x.to_string();
  let x_chars = st.as_bytes().iter().map(|&item| item as char).collect::<Vec<char>>();
  for i in 0..x_chars.len() / 2 {
    let item = x_chars[i];
    let next_part_item = x_chars[x_chars.len() - 1 - i];
    if item != next_part_item {
      return false;
    }
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let x = 121;
    assert!(is_palindrome(x));
  }
}
