// refer to https://leetcode.com/problems/leaf-similar-trees/

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

fn get_last_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if let Some(node) = root {
        let mut result = vec![];
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none() {
            result.push(node.val);
        }
        if node.left.is_some() {
            let mut left = get_last_values(node.left.clone());
            result.append(&mut left);
        }
        if node.right.is_some() {
            let mut right = get_last_values(node.right.clone());
            result.append(&mut right);
        }
        result
    } else {
        vec![]
    }
}

pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    get_last_values(root1) == get_last_values(root2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // root1 = [3,5,1,6,2,9,8,null,null,7,4], root2 = [3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]
        assert!(leaf_similar(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),

                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                    }))),
                }))),
            }))),
        ));
    }
}
