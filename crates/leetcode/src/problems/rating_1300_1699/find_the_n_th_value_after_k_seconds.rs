// https://leetcode.com/problems/find-the-n-th-value-after-k-seconds

pub struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn pascal_triangle(row_index: usize) -> Vec<i64> {
        let mut row = vec![1_i64];

        for _ in 0..row_index {
            let mut next = vec![1_i64; row.len() + 1];
            for i in 1..row.len() {
                next[i] = (row[i - 1] + row[i]) % Self::MOD;
            }
            row = next;
        }

        row
    }

    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        let row_index = (n + k - 1) as usize;
        let pick = k as usize;
        let triangle = Self::pascal_triangle(row_index);
        triangle[pick] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::value_after_k_seconds(4, 5), 56);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::value_after_k_seconds(5, 3), 35);
    }
}
