#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if l1.is_none() {
        return l2;
    }
    if l2.is_none() {
        return l1;
    }
    let mut head = Some(Box::new(ListNode { val: 0, next: None }));
    let mut l1_clone: Option<Box<ListNode>> = l1.clone();
    let mut l2_clone: Option<Box<ListNode>> = l2.clone();

    let mut tail_head = head.as_mut();
    let mut tail_1 = l1_clone.as_mut();
    let mut tail_2 = l2_clone.as_mut();
    let mut is_increase = false;
    loop {
        let tail_1_some = tail_1.is_some();
        let tail_2_some = tail_2.is_some();
        if !tail_1_some && !tail_2_some {
            break;
        }
        let node_1_num = if tail_1_some {
            let node_1 = tail_1.unwrap();
            tail_1 = node_1.next.as_mut();
            node_1.val
        } else {
            tail_1 = None;
            0
        };
        let node_2_num = if tail_2_some {
            let node_2 = tail_2.unwrap();
            tail_2 = node_2.next.as_mut();
            node_2.val
        } else {
            tail_2 = None;
            0
        };
        let mut sum = if is_increase {
            node_1_num + node_2_num + 1
        } else {
            node_1_num + node_2_num
        };
        let node_head: &mut Box<ListNode> = tail_head.unwrap();
        if sum >= 10 {
            is_increase = true;
            sum -= 10;
        } else {
            is_increase = false;
        }
        if tail_2.is_some() || tail_1.is_some() || is_increase {
            node_head.next = Some(Box::new(ListNode {
                val: if is_increase { 1 } else { 0 },
                next: None,
            }));
        } else {
            node_head.next = None;
        }
        node_head.val = sum;
        tail_head = node_head.next.as_mut();
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            add_two_numbers(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 3, next: None }))
                        }))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 8,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                }))
            }))
        )
    }
}
