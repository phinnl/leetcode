pub fn license_key_formatting(s: String, k: i32) -> String {
    let s = s.to_uppercase().replace('-', "");
    let mut result = String::new();
    let mut cur = s.as_str();
    while !cur.is_empty() {
        let (chunk, rest) =
            cur.split_at(std::cmp::max(0, cur.len() as isize - k as isize) as usize);
        if result.is_empty() {
            rest.clone_into(&mut result);
        } else {
            result = format!("{}-{}", rest, result);
        }
        cur = chunk;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            license_key_formatting("5F3Z-2e-9-w".to_owned(), 4),
            "5F3Z-2E9W"
        );
    }
}
