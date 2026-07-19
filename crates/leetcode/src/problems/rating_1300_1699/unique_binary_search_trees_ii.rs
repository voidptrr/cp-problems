// https://leetcode.com/problems/unique-binary-search-trees-ii

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
    fn build(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }

        let mut result = Vec::new();

        for root_val in start..=end {
            let left_trees = Self::build(start, root_val - 1);
            let right_trees = Self::build(root_val + 1, end);

            for left in &left_trees {
                for right in &right_trees {
                    let root = TreeNode {
                        val: root_val,
                        left: left.clone(),
                        right: right.clone(),
                    };

                    result.push(Some(Rc::new(RefCell::new(root))));
                }
            }
        }

        result
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::build(1, n)
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    fn serialize(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        let mut queue = VecDeque::from([root.clone()]);

        while let Some(node) = queue.pop_front() {
            if let Some(node) = node {
                let borrowed = node.borrow();
                result.push(Some(borrowed.val));
                queue.push_back(borrowed.left.clone());
                queue.push_back(borrowed.right.clone());
            } else {
                result.push(None);
            }
        }

        while result.last() == Some(&None) {
            result.pop();
        }

        result
    }

    fn sorted_serialized_trees(
        trees: Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> Vec<Vec<Option<i32>>> {
        let mut serialized = trees.iter().map(serialize).collect::<Vec<_>>();
        serialized.sort();
        serialized
    }

    #[test]
    fn example_one() {
        let expected = vec![
            vec![Some(1), None, Some(2), None, Some(3)],
            vec![Some(1), None, Some(3), Some(2)],
            vec![Some(2), Some(1), Some(3)],
            vec![Some(3), Some(1), None, None, Some(2)],
            vec![Some(3), Some(2), None, Some(1)],
        ];

        assert_eq!(
            sorted_serialized_trees(Solution::generate_trees(3)),
            sorted_serialized_trees_from_values(expected),
        );
    }

    #[test]
    fn example_two() {
        let expected = vec![vec![Some(1)]];

        assert_eq!(
            sorted_serialized_trees(Solution::generate_trees(1)),
            sorted_serialized_trees_from_values(expected),
        );
    }

    fn sorted_serialized_trees_from_values(
        mut values: Vec<Vec<Option<i32>>>,
    ) -> Vec<Vec<Option<i32>>> {
        values.sort();
        values
    }
}
