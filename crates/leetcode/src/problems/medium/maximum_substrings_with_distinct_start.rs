// https://leetcode.com/problems/maximum-substrings-with-distinct-start

pub struct Solution;

impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let mut seen = [false; 26];
        let mut count = 0;

        for &b in s.as_bytes() {
            let idx = (b - b'a') as usize;
            if !seen[idx] {
                seen[idx] = true;
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::max_distinct("abab".to_owned()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::max_distinct("abcd".to_owned()), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::max_distinct("aaaa".to_owned()), 1);
    }
}
