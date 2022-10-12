/// https://leetcode.com/problems/longest-substring-without-repeating-characters/
/// Given a string s, find the length of the longest substring without repeating characters.
use super::common::solution::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(str: String) -> i32 {
        let mut max_len = 0;
        let mut len = 0;
        let mut map = HashMap::new();
        for (i, chr) in str.chars().enumerate() {
            let old_pos = map.entry(chr).or_insert(i);
            if *old_pos == i || i - *old_pos > len {
                len += 1;
            } else {
                max_len = max_len.max(len);
                len = i - *old_pos
            }

            *old_pos = i;
        }

        max_len.max(len) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("abcabcbb"))
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring(String::from("bbbbb"))
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("pwwkew"))
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("dvdf"))
        );
    }
}
