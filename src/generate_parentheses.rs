fn push_parenthesis(s: String, left: i32, right: i32) -> Vec<String> {
  let a = match s.as_str() {
      "" => push_parenthesis("(".to_string(), left - 1, right),
      _ => {
          let mut result: Vec<String> = Vec::new();
          if left == 0 && right == 0 {
              result.push(s);
              return result;
          }
          if left > 0 {
              let mut vec_left = push_parenthesis(format!("{s}("), left - 1, right);
              result.append(&mut vec_left);
          }
          if right > 0 && left < right {
              let mut vec_right = push_parenthesis(format!("{s})"), left, right - 1);
              result.append(&mut vec_right);
          }
          result
      }
  };
  a
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
  push_parenthesis("".to_string(), n, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n = 3;
        let output = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(generate_parenthesis(n), output);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let output = vec!["()"];
        assert_eq!(generate_parenthesis(n), output);
    }

    #[test]
    fn example_3() {
        let n = 2;
        let output = vec!["(())", "()()"];
        assert_eq!(generate_parenthesis(n), output);
    }
}