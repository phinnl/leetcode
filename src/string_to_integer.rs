pub fn my_atoi(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut temp = String::new();
    for (index, &i_byte) in bytes.iter().enumerate() {
        let item = i_byte as char;
        let next_item = bytes.get(index + 1);
        if item.is_numeric()
            || ((item == '-' || item == '+')
                && next_item.is_some()
                && (*next_item.unwrap() as char).is_numeric()
                && temp.is_empty())
        {
            temp = format!("{temp}{item}");
        } else if item == ' ' {
            continue;
        } else {
            break;
        }
        if next_item.is_none() || !(*next_item.unwrap() as char).is_numeric() {
            break;
        }
    }

    let temp_isize = temp.parse::<isize>().unwrap_or(0);
        println!("{temp} {temp_isize}");
    if temp_isize > i32::MAX as isize {
        i32::MAX
    } else if temp_isize < i32::MIN as isize {
        i32::MIN
    } else {
        temp_isize as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "20000000000000000000".to_owned();
        assert_eq!(412, my_atoi(s));
    }
}
