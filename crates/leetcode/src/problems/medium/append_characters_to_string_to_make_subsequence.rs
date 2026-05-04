// https://leetcode.com/problems/append-characters-to-string-to-make-subsequence

pub struct Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        while left < s.len() && right < t.len() {
            if s_bytes[left] == t_bytes[right] {
                left += 1;
                right += 1;
            } else {
                left += 1;
            }
        }

        (t.len() - right) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::append_characters(
                "coaching".to_owned(),
                "coding".to_owned()
            ),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::append_characters("abcde".to_owned(), "a".to_owned()),
            0
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::append_characters("z".to_owned(), "abcde".to_owned()),
            5
        );
    }
}
