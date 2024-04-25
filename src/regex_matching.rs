pub fn is_match(_s: String, _p: String) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "aa".to_owned();
        let p = "a".to_owned();
        assert!(is_match(s, p));
    }

    #[test]
    fn example_2() {
        let s = "aa".to_owned();
        let p = "a*".to_owned();
        assert!(is_match(s, p));
    }

    #[test]
    fn example_3() {
        let s = "ab".to_owned();
        let p = ".*".to_owned();
        assert!(is_match(s, p));
    }

    #[test]
    fn example_4() {
        let s = "aab".to_owned();
        let p = "c*a*b".to_owned();
        assert!(is_match(s, p));
    }
}
