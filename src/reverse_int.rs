pub fn reverse(num: i32) -> i32 {
    let is_positive = num > 0;
    match num
        .to_string()
        .replace('-', "")
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
    {
        Ok(num_rev) => {
            if !is_positive {
                -num_rev
            } else {
                num_rev
            }
        }
        Err(_err) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn example_2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn example_3() {
        assert_eq!(reverse(1534236469), 0);
    }
}
