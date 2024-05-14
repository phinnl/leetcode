#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

fn get_left_values(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> Vec<i32> {
    let mut result = Vec::new();
    if let Some(node) = root {
        if node.borrow().left.is_some() || node.borrow().right.is_some() {
            result.extend(get_left_values(node.borrow().left.clone(), true));
            result.extend(get_left_values(node.borrow().right.clone(), false));
        } else if is_left {
            result.push(node.borrow().val);
        }
    }
    result
}

// refer to https://leetcode.com/problems/sum-of-left-leaves/

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    get_left_values(root, false).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            sum_of_left_leaves(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                })))
            })))),
            24
        );
    }
}
