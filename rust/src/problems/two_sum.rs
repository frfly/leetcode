/// https://leetcode.com/problems/two-sum/
/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
/// You can return the answer in any order.
use std::collections::HashMap;

use super::common::solution::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (ind, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(pos) => return vec![*pos as i32, ind as i32],
                None => {
                    map.insert(num, ind);
                }
            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }
}
