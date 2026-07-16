// https://leetcode.com/problems/minimum-absolute-difference-in-bst

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[must_use]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    fn inorder(
        node: &Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<i32>,
        min: &mut i32,
    ) {
        let Some(node) = node else {
            return;
        };

        let borrowed = node.borrow();
        Self::inorder(&borrowed.left, prev, min);

        if let Some(prev_val) = *prev {
            *min = (*min).min(borrowed.val - prev_val);
        }
        *prev = Some(borrowed.val);

        Self::inorder(&borrowed.right, prev, min);
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut prev = None;
        let mut min = i32::MAX;

        Self::inorder(&root, &mut prev, &mut min);
        min
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn example_one() {
        let root = node(
            4,
            node(2, node(1, None, None), node(3, None, None)),
            node(6, None, None),
        );

        assert_eq!(Solution::get_minimum_difference(root), 1);
    }

    #[test]
    fn example_two() {
        let root = node(
            1,
            node(0, None, None),
            node(48, node(12, None, None), node(49, None, None)),
        );

        assert_eq!(Solution::get_minimum_difference(root), 1);
    }
}
