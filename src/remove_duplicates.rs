#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // nums is sorted and the idea is to move already present numbers to the end of the vec in a
    // arbitrary order. Iterate over vec once and remember prev number + arr len as the k value. If prev number == curr
    // number, move to the end of vec and decrease k by one
    let mut prev: Option<i32> = None;
    let mut indices: Vec<usize> = vec![];
    for (ind, val) in nums.iter_mut().enumerate() {
        if ind == 0 {
            prev = Some(*val);
            continue;
        }
        if *val == prev.unwrap() {
            indices.push(ind);
        }
        prev = Some(*val);
    }
    indices.reverse();
    let mut k = nums.len();
    for i in indices {
        let val = nums.remove(i);
        nums.push(val);
        k -= 1
    }
    println!("{:?}", nums);
    k as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_duplicates_short() {
        let mut nums = vec![1, 1, 2];
        let res = remove_duplicates(&mut nums);
        assert_eq!(res, 2);
    }
    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let res = remove_duplicates(&mut nums);
        assert_eq!(res, 5);
    }
}
