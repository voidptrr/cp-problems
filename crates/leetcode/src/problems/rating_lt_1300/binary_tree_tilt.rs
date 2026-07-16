// https://leetcode.com/problems/binary-tree-tilt

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
    fn subtree_sum(
        node: &Option<Rc<RefCell<TreeNode>>>,
        total_tilt: &mut i32,
    ) -> i32 {
        let Some(node) = node else {
            return 0;
        };

        let borrowed = node.borrow();
        let left_sum = Self::subtree_sum(&borrowed.left, total_tilt);
        let right_sum = Self::subtree_sum(&borrowed.right, total_tilt);

        *total_tilt += (left_sum - right_sum).abs();

        left_sum + right_sum + borrowed.val
    }

    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let _ = Self::subtree_sum(&root, &mut result);

        result
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
        let root = node(1, node(2, None, None), node(3, None, None));

        assert_eq!(Solution::find_tilt(root), 1);
    }

    #[test]
    fn example_two() {
        let root = node(
            4,
            node(2, node(3, None, None), node(5, None, None)),
            node(9, None, node(7, None, None)),
        );

        assert_eq!(Solution::find_tilt(root), 15);
    }

    #[test]
    fn example_three() {
        let root = node(
            21,
            node(
                7,
                node(1, node(3, None, None), node(3, None, None)),
                node(1, None, None),
            ),
            node(14, node(2, None, None), node(2, None, None)),
        );

        assert_eq!(Solution::find_tilt(root), 9);
    }
}
