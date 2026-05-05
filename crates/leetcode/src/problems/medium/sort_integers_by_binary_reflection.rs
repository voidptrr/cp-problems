// https://leetcode.com/problems/sort-integers-by-binary-reflection

pub struct Solution;

impl Solution {
    fn revert(mut x: u32) -> u32 {
        let mut r = 0u32;
        while x > 0 {
            r = (r << 1) | (x & 1);
            x >>= 1;
        }
        r
    }

    pub fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums;
        result.sort_by(|a, b| {
            let ra = Self::revert(*a as u32);
            let rb = Self::revert(*b as u32);
            ra.cmp(&rb).then(a.cmp(b))
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::sort_by_reflection(vec![4, 5, 4]), vec![4, 4, 5]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::sort_by_reflection(vec![3, 6, 5, 8]),
            vec![8, 3, 6, 5]
        );
    }
}
