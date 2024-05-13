// refer to https://leetcode.com/problems/palindrome-number/

pub fn is_palindrome(x: i32) -> bool {
    let x = x.to_string();
    let arr = x.as_bytes();
    for index in 0..arr.len() / 2 {
        if arr[index] != arr[arr.len() - 1 - index] {
            return false;
        }
    }
    true
}

pub fn is_palindrome_2(x: i32) -> bool {
    x.to_string() == x.to_string().chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let x = 121;
        assert!(is_palindrome(x));
        assert!(is_palindrome_2(x));
    }
}
