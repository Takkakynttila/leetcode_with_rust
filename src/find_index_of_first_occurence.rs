#[allow(dead_code)]
pub fn str_str(haystack: String, needle: String) -> i32 {
    let res = haystack.find(&needle);
    if res.is_none() {
        return -1;
    }
    res.unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str_success() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let res = str_str(haystack, needle);
        assert_eq!(res, 0)
    }

    #[test]
    fn test_str_str_failure() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        let res = str_str(haystack, needle);
        assert_eq!(res, -1)
    }
}
