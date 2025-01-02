#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

#[allow(dead_code)]
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[allow(dead_code)]
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    use std::collections::BinaryHeap;

    fn extract_to_heap(listnode: Option<Box<ListNode>>, result: &mut BinaryHeap<i32>) {
        let mut next = listnode;
        while next.is_some() {
            let val = next.clone().unwrap().val;
            result.push(val);
            next = next.unwrap().next
        }
    }

    let mut result:BinaryHeap<i32> = BinaryHeap::new();

    extract_to_heap(list1, &mut result);
    extract_to_heap(list2, &mut result);

    let mut prev:Option<Box<ListNode>> = None;
    loop {
        let max_value = result.pop();
        if max_value.is_none() {
            break;
        }
        if prev.is_none() {
            prev = Some(Box::new(ListNode{val: max_value.unwrap(), next:None}));
            continue;
        }
        prev = Some(Box::new(ListNode{val: max_value.unwrap(), next:prev}));

    } 
    prev
    }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_merge_two_lists_success() {
        let l3 = Box::new(ListNode{val: 4, next: None});
        let l2 = Box::new(ListNode{val: 2, next: Some(l3)});
        let l1 = Box::new(ListNode{val: 1, next: Some(l2)});

        let n3 = Box::new(ListNode{val: 3, next: None});
        let n2 = Box::new(ListNode{val: 2, next: Some(n3)});
        let n1 = Box::new(ListNode{val: 1, next: Some(n2)});
        let _res = merge_two_lists(Some(l1), Some(n1));
    }
}
