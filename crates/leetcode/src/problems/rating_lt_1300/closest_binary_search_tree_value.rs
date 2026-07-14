// https://leetcode.com/problems/closest-binary-search-tree-value

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
    pub fn closest_value(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: f64,
    ) -> i32 {
        let Some(root) = root else {
            panic!("Invalid input");
        };

        let mut min: f64 = (target - root.borrow().val as f64).abs();
        let mut result = root.borrow().val;
        let mut stack = vec![root];

        while let Some(node) = stack.pop() {
            let borrowed = node.borrow();
            let diff = (target - borrowed.val as f64).abs();

            if diff < min || (diff == min && borrowed.val < result) {
                result = borrowed.val;
                min = diff;
            };

            if let Some(left) = &borrowed.left {
                stack.push(Rc::clone(left));
            }

            if let Some(right) = &borrowed.right {
                stack.push(Rc::clone(right));
            }
        }

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
        let root = node(
            4,
            node(2, node(1, None, None), node(3, None, None)),
            node(5, None, None),
        );

        assert_eq!(Solution::closest_value(root, 3.714_286), 4);
    }

    #[test]
    fn example_two() {
        let root = node(1, None, None);

        assert_eq!(Solution::closest_value(root, 4.428_571), 1);
    }

    #[test]
    fn equal_distance_chooses_smaller_value() {
        let root = node(
            4,
            node(2, node(1, None, None), node(3, None, None)),
            node(5, None, None),
        );

        assert_eq!(Solution::closest_value(root, 3.5), 3);
    }
}
