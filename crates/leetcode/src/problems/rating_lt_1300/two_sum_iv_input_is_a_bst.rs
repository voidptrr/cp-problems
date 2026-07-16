// https://leetcode.com/problems/two-sum-iv-input-is-a-bst

use std::cell::RefCell;
use std::collections::HashSet;
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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let Some(root) = root else {
            return false;
        };

        let mut set = HashSet::<i32>::new();
        let mut stack = vec![root];

        while let Some(node) = stack.pop() {
            let current = node.borrow();
            if set.contains(&(k - current.val)) {
                return true;
            }

            set.insert(current.val);

            if let Some(left) = &current.left {
                stack.push(left.clone());
            }

            if let Some(right) = &current.right {
                stack.push(right.clone());
            }
        }

        false
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

    fn example_tree() -> Option<Rc<RefCell<TreeNode>>> {
        node(
            5,
            node(3, node(2, None, None), node(4, None, None)),
            node(6, None, node(7, None, None)),
        )
    }

    #[test]
    fn example_one() {
        assert!(Solution::find_target(example_tree(), 9));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::find_target(example_tree(), 28));
    }
}
