// refer to https://leetcode.com/problems/zigzag-conversion/

pub fn convert(s: String, num_rows: i32) -> String {
    let len = s.len();
    if len as i32 <= num_rows || num_rows == 1 {
        return s;
    }
    let mut output = String::new();
    let s_bytes = s.as_bytes();
    for index in 0..num_rows {
        let mut cur_index = index;
        while cur_index < len as i32 {
            output.push(*s_bytes.get(cur_index as usize).unwrap() as char);
            if index != 0 && (index + 1) % num_rows != 0 {
                let op = s_bytes
                    .get((cur_index - index) as usize + num_rows as usize * 2 - 2 - index as usize);
                if let Some(it) = op {
                    output.push(*it as char);
                }
            }
            cur_index += num_rows * 2 - 2;
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        assert_eq!("PINALSIGYAHRPI".to_owned(), convert(s, num_rows));
    }
}
