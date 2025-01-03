#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    use std::collections::VecDeque;
    let mut deque = VecDeque::new();
    let mut target: Option<char>;
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => deque.push_front(c),
            ')' => {
                target = deque.pop_front();

                if target.is_none() {
                    return false;
                }

                if target.unwrap() != '(' {
                    return false;
                }
            }
            ']' => {
                target = deque.pop_front();

                if target.is_none() {
                    return false;
                }

                if target.unwrap() != '[' {
                    return false;
                }
            }
            '}' => {
                target = deque.pop_front();

                if target.is_none() {
                    return false;
                }

                if target.unwrap() != '{' {
                    return false;
                }
            }
            _ => {}
        }
    }
    deque.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let input = "()".to_string();
        let result = is_valid(input);
        assert!(result);
    }

    #[test]
    fn test_is_valid_mixed() {
        let input = "([])".to_string();
        let result = is_valid(input);
        assert!(result);
    }

    #[test]
    fn test_is_invalid_mixed() {
        let input = "((){[}])".to_string();
        let result = is_valid(input);
        assert!(!result);
    }

    #[test]
    fn test_is_invalid() {
        let input = "(]".to_string();
        let result = is_valid(input);
        assert!(!result);
    }

    #[test]
    fn test_is_invalid_single_closing() {
        let input = "]".to_string();
        let result = is_valid(input);
        assert!(!result);
    }

    #[test]
    fn test_is_invalid_single_opening() {
        let input = "{".to_string();
        let result = is_valid(input);
        assert!(!result);
    }
}
