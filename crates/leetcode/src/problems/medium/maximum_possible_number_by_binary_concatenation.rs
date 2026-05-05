// https://leetcode.com/problems/maximum-possible-number-by-binary-concatenation

pub struct Solution;

impl Solution {
    fn concat(a: i32, b: i32, c: i32) -> i32 {
        (((a << (32 - b.leading_zeros())) | b) << (32 - c.leading_zeros())) | c
    }

    pub fn max_good_number(nums: Vec<i32>) -> i32 {
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];

        let v1 = Self::concat(a, b, c);
        let v2 = Self::concat(a, c, b);
        let v3 = Self::concat(b, a, c);
        let v4 = Self::concat(b, c, a);
        let v5 = Self::concat(c, a, b);
        let v6 = Self::concat(c, b, a);

        *[v1, v2, v3, v4, v5, v6].iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::max_good_number(vec![1, 2, 3]), 30);
    }

    #[test]
    fn custom_one() {
        assert_eq!(Solution::max_good_number(vec![0, 0, 0]), 0);
    }

    #[test]
    fn custom_two() {
        assert_eq!(Solution::max_good_number(vec![1, 1, 1]), 7);
    }
}
