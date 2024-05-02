use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut result: i32 = 0;
    let roman_map: HashMap<char, u32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let s_bytes = s.as_bytes();
    for index in 0..s_bytes.len() {
        let item = s_bytes[index] as char;
        let integer = roman_map.get(&item).unwrap();
        if let Some(str) = s_bytes.get(index + 1) {
            let next_item = roman_map.get(&(*str as char)).unwrap();
            if next_item > integer {
                result -= *integer as i32;
                continue;
            }
        }
        result += *integer as i32;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "III".to_string();
        assert_eq!(3, roman_to_int(s));
    }

    #[test]
    fn example_2() {
        let s = "LVIII".to_string();
        assert_eq!(58, roman_to_int(s));
    }

    #[test]
    fn example_3() {
        let s = "MCMXCIV".to_string();
        assert_eq!(1994, roman_to_int(s));
    }
}
