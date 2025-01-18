#[allow(dead_code)]
pub fn my_sqrt(x: i32) -> i32 {
    (x as f64).sqrt() as i32 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        let x = 4;
        let res = my_sqrt(x);
        assert_eq!(res, 2)
    }

    #[test]
    fn test_my_sqrt_eight() {
        let x = 8;
        let res = my_sqrt(x);
        assert_eq!(res, 2)

    }
}
