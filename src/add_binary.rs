#[allow(dead_code)]
pub fn add_binary(a: String, b: String) -> String {
    use std::cmp;
    let max_len = cmp::max(a.len(), b.len());
    let a_padded = format!("{:0>pad$}", a.clone(), pad = max_len)
        .chars()
        .rev()
        .collect::<Vec<char>>();
    let b_padded = format!("{:0>pad$}", b.clone(), pad = max_len)
        .chars()
        .rev()
        .collect::<Vec<char>>();

    // if 0, add 0, if 1 add 1, if 2 add 0 carry 1 if 3 add 1 carry 1
    let mut carry = 0_i32;
    let mut result: Vec<i32> = vec![];
    for i in 0..max_len {
        let a_char = a_padded[i] as i32 - 0x30;
        let b_char = b_padded[i] as i32 - 0x30;
        let operation = a_char + b_char + carry;
        match operation {
            0 | 1 => {
                result.push(operation);
                carry = 0;
            }
            2 => {
                result.push(0);
                carry = 1;
            }
            3 => {
                result.push(1);
                carry = 1;
            }
            _ => {}
        }
    }
    if carry > 0 {
        result.push(carry);
    }
    result.iter().rev().map(ToString::to_string).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        let a = "11".to_string();
        let b = "1".to_string();
        let res = add_binary(a, b);
        assert_eq!(res, "100".to_string())
    }

    #[test]
    fn test_add_binary_long() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let res = add_binary(a, b);
        assert_eq!(res, "10101".to_string())
    }
}
