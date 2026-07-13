// https://leetcode.com/problems/keys-and-rooms
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    fn dfs_graph(rooms: &[Vec<i32>], set: &mut HashSet<i32>) {
        let mut stack = vec![0usize];
        set.insert(0);

        while let Some(room) = stack.pop() {
            let current_door = &rooms[room];
            for key in current_door {
                if set.insert(*key) {
                    stack.push(*key as usize);
                }
            }
        }
    }

    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut keys = HashSet::<i32>::new();

        Self::dfs_graph(&rooms, &mut keys);

        keys.len() == rooms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert!(Solution::can_visit_all_rooms(vec![
            vec![1],
            vec![2],
            vec![3],
            vec![],
        ]));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::can_visit_all_rooms(vec![
            vec![1, 3],
            vec![3, 0, 1],
            vec![2],
            vec![0],
        ]));
    }
}
