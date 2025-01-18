#[allow(dead_code)]
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 0_i32;
    let mut export: Vec<i32> = vec![];
    let mut incr = 1_i32;
    for i in digits.into_iter().rev() {
        let mut target = i + carry + incr;
        incr = 0;
        carry = 0;
        if target >= 10 {
            target -= 10;
            carry = 1;
        }
        export.push(target);
    }
    if carry == 1 {
        export.push(1)
    }
    export.reverse();
    export
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plus_one_single_digit() {
        let digits = vec![9];
        let output = vec![1, 0];
        let res = plus_one(digits);
        assert_eq!(res, output)
    }

    #[test]
    fn test_plus_one() {
        let digits = vec![1, 2, 3];
        let output = vec![1, 2, 4];
        let res = plus_one(digits);
        assert_eq!(res, output)
    }
    #[test]
    fn test_plus_one_larger() {
        let digits = vec![4, 3, 2, 1];
        let output = vec![4, 3, 2, 2];
        let res = plus_one(digits);
        assert_eq!(res, output)
    }

    #[test]
    fn test_plus_one_larger_really_large() {
        let digits = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let output = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1];
        let res = plus_one(digits);
        assert_eq!(res, output)
    }
}
