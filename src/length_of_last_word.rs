#[allow(dead_code)]
pub fn length_of_last_word(s: String) -> i32 {
    let s_trimmed = s.trim().to_string();
    let s_vec: Vec<&str> = s_trimmed.split(" ").collect();
    s_vec.last().unwrap().len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        let s = "Hello World".to_string();
        let res = length_of_last_word(s);
        assert_eq!(res, 5)
    }

    #[test]
    fn test_length_of_last_word_extra_whitespaces() {
        let s = "   fly me   to   the moon  ".to_string();
        let res = length_of_last_word(s);
        assert_eq!(res, 4)
    }

    #[test]
    fn test_length_of_last_word_long_word() {
        let s = "luffy is still joyboy".to_string();
        let res = length_of_last_word(s);
        assert_eq!(res, 6)
    }
}
