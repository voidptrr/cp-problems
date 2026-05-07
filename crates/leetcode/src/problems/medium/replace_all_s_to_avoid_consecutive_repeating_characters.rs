// https://leetcode.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters

pub struct Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut out = s.into_bytes();

        for i in 0..out.len() {
            if out[i] == b'?' {
                for ch in b'a'..=b'z' {
                    let ok_left = i == 0 || out[i - 1] != ch;
                    let ok_right = i + 1 == out.len()
                        || out[i + 1] == b'?'
                        || out[i + 1] != ch;

                    if ok_left && ok_right {
                        out[i] = ch;
                        break;
                    }
                }
            }
        }

        String::from_utf8(out).expect("valid lowercase output")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        let out = Solution::modify_string("?zs".to_owned());
        assert_eq!(out.len(), 3);
        assert_eq!(out.as_bytes()[1], b'z');
        assert_eq!(out.as_bytes()[2], b's');
        for w in out.as_bytes().windows(2) {
            assert_ne!(w[0], w[1]);
        }
    }

    #[test]
    fn example_two() {
        let out = Solution::modify_string("ubv?w".to_owned());
        assert_eq!(out.len(), 5);
        assert_eq!(out.as_bytes()[0], b'u');
        assert_eq!(out.as_bytes()[1], b'b');
        assert_eq!(out.as_bytes()[2], b'v');
        assert_eq!(out.as_bytes()[4], b'w');
        for w in out.as_bytes().windows(2) {
            assert_ne!(w[0], w[1]);
        }
    }
}
