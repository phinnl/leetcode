pub fn length_of_longest_substring(s: String) -> i32 {
    let mut length = 0;
    let mut st = "".to_owned();
    for item in s.chars() {
        if let Some(index) = st.find(item) {
            st = st[index + 1..].to_owned();
            st.push(item);
            let new_length = st.len() as i32;
            if new_length > length {
                length = new_length;
            }
            continue;
        }
        st.push(item);
        let new_length = st.len() as i32;
        if new_length > length {
            length = new_length;
        }
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        let input = "abcabcbb".to_owned();
        let output = length_of_longest_substring(input.clone());
        let expect = 3;
        assert_eq!(
            output, expect,
            "Input: s = {input} expect = {expect} and output = {output}",
        );
    }
    #[test]
    fn example_2() {
        let input = "bbbbb".to_owned();
        let output = length_of_longest_substring(input.clone());
        let expect = 1;
        assert_eq!(
            output, expect,
            "Input: s = {input} expect = {expect} and output = {output}",
        );
    }
    #[test]
    fn example_3() {
        let input = "pwwkew".to_owned();
        let output = length_of_longest_substring(input.clone());
        let expect = 3;
        assert_eq!(
            output, expect,
            "Input: s = {input} expect = {expect} and output = {output}",
        );
    }
}
