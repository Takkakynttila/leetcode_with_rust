#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut counter = 0;
    for n in nums.into_iter() {
        if n >= target {
            return counter;
        }
        counter += 1
    }
    counter
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_search_insert_in_list() {
        let nums = vec![1, 3, 5, 6];
        let target = 5_i32;
        let res = search_insert(nums, target);
        assert_eq!(res, 2)
    }

    #[test]
    fn test_search_insert_not_in_list() {
        let nums = vec![1, 3, 5, 6];
        let target = 2_i32;
        let res = search_insert(nums, target);
        assert_eq!(res, 1)
    }

    #[test]
    fn test_search_insert_outside_of_range() {
        let nums = vec![1, 3, 5, 6];
        let target = 7_i32;
        let res = search_insert(nums, target);
        assert_eq!(res, 4)
    }
}
