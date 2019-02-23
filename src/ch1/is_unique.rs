use std::collections::HashSet;

/// Determine if a STRING has all unique characters.
pub fn is_unique_chars(string: &str) -> bool {
    let mut data = HashSet::new();

    for c in string.chars() {
        match data.get(&c) {
            Some(_) => return false,
            None => data.insert(c),
        };
    }

    true
}

/// Same as is_unique, but without data structures.
/// Assume STRING contains only ascii characters.
pub fn is_unique_chars_no_data_structure(string: &str) -> bool {
    let len = string.len();
    for i in 0..len {
        let mut rem = string[i..].chars();
        let ch = rem.next().unwrap();
        for ch2 in rem {
            if ch == ch2 {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert!(is_unique_chars(""));
        assert!(is_unique_chars("abc"));
        assert!(!is_unique_chars("aaa"));
        assert!(is_unique_chars("ほげ"));
        assert!(!is_unique_chars("ほげほげ"));
    }

    #[test]
    fn test_is_unique_no_data_structure() {
        assert!(is_unique_chars_no_data_structure(""));
        assert!(is_unique_chars_no_data_structure("abc"));
        assert!(!is_unique_chars_no_data_structure("aaa"));
    }

}
