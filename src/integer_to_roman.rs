// refer to https://leetcode.com/problems/integer-to-roman/

pub fn int_to_roman(num: i32) -> String {
    const RADIX: u32 = 10;
    let roman = ["I", "V", "X", "L", "C", "D", "M"];
    let mut result = String::new();
    let nums = num.to_string()
        .as_bytes()
        .iter()
        .map(|&item| (item as char).to_digit(RADIX).unwrap())
        .collect::<Vec<u32>>();
    for index in 0..nums.len() {
        let item = nums[nums.len() - 1 - index];
        match item {
            1..=3 => {
                for _i in 0..item {
                    result.insert_str(0, roman[index * 2]);
                }
            }
            4..=8 => {
                let mut temp_str = roman[index * 2 + 1].to_owned();
                if item == 4 {
                    temp_str.insert_str(0, roman[index * 2]);
                }
                if item > 5 {
                    for _i in 0..item - 5 {
                        temp_str.push_str(roman[index * 2]);
                    }
                }
                result.insert_str(0, &temp_str);
            }
            _ => {
              if item != 0 {
                result.insert_str(0, &format!("{}{}", roman[index * 2], roman[index * 2 + 2]));
              }
            }
        }
    }
    result
}

// 1 5 10 50 100 500 1000
// I V X  L  C   D   M

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let num = 3;
        assert_eq!("III", int_to_roman(num));
    }

    #[test]
    fn example_2() {
        let num = 58;
        assert_eq!("LVIII", int_to_roman(num));
    }
    
    #[test]
    fn example_3() {
        let num = 1994;
        assert_eq!("MCMXCIV", int_to_roman(num));
    }
}
