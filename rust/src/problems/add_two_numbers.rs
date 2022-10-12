use super::common::list_node::ListNode;
///https://leetcode.com/problems/add-two-numbers/
///You are given two non-empty linked lists representing two non-negative integers.
///The digits are stored in reverse order, and each of their nodes contains a single digit.
///Add the two numbers and return the sum as a linked list.
///You may assume the two numbers do not contain any leading zero, except the number 0 itself.
use super::common::solution::Solution;

impl Solution {
    pub fn add_two_numbers(
        left: Option<Box<ListNode>>,
        right: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut left_tail = &left;
        let mut right_tail = &right;
        let mut head = Option::Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        let mut rest = 0;
        loop {
            if let Some(x) = match (left_tail, right_tail, rest) {
                (None, None, 0) => (Option::None),
                (None, None, x) => {
                    rest = 0;
                    Option::Some(x)
                }
                (None, Some(r), x) => {
                    right_tail = &r.next;
                    let sum = x + r.val;
                    rest = sum / 10;
                    Option::Some(sum % 10)
                }
                (Some(l), None, x) => {
                    left_tail = &l.next;
                    let sum = x + l.val;
                    rest = sum / 10;
                    Option::Some(sum % 10)
                }
                (Some(l), Some(r), x) => {
                    right_tail = &r.next;
                    left_tail = &l.next;
                    let sum = x + r.val + l.val;
                    rest = sum / 10;
                    Option::Some(sum % 10)
                }
            } {
                let child = ListNode::new(x);
                tail.as_mut()?.next = Option::Some(Box::new(child));
                tail = &mut tail.as_mut()?.next;
            } else {
                return head?.next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Option::Some(Box::new(ListNode {
                val: 7,
                next: Option::Some(Box::new(ListNode {
                    val: 0,
                    next: Option::Some(Box::new(ListNode {
                        val: 8,
                        next: Option::None
                    }))
                }))
            })),
            Solution::add_two_numbers(
                Option::Some(Box::new(ListNode {
                    val: 2,
                    next: Option::Some(Box::new(ListNode {
                        val: 4,
                        next: Option::Some(Box::new(ListNode {
                            val: 3,
                            next: Option::None
                        }))
                    }))
                })),
                Option::Some(Box::new(ListNode {
                    val: 5,
                    next: Option::Some(Box::new(ListNode {
                        val: 6,
                        next: Option::Some(Box::new(ListNode {
                            val: 4,
                            next: Option::None
                        }))
                    }))
                }))
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Option::Some(Box::new(ListNode {
                val: 0,
                next: Option::None
            })),
            Solution::add_two_numbers(
                Option::Some(Box::new(ListNode {
                    val: 0,
                    next: Option::None
                })),
                Option::Some(Box::new(ListNode {
                    val: 0,
                    next: Option::None
                }))
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Option::Some(Box::new(ListNode {
                val: 8,
                next: Option::Some(Box::new(ListNode {
                    val: 9,
                    next: Option::Some(Box::new(ListNode {
                        val: 9,
                        next: Option::Some(Box::new(ListNode {
                            val: 9,
                            next: Option::Some(Box::new(ListNode {
                                val: 0,
                                next: Option::Some(Box::new(ListNode {
                                    val: 0,
                                    next: Option::Some(Box::new(ListNode {
                                        val: 0,
                                        next: Option::Some(Box::new(ListNode {
                                            val: 1,
                                            next: Option::None
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            })),
            Solution::add_two_numbers(
                Option::Some(Box::new(ListNode {
                    val: 9,
                    next: Option::Some(Box::new(ListNode {
                        val: 9,
                        next: Option::Some(Box::new(ListNode {
                            val: 9,
                            next: Option::Some(Box::new(ListNode {
                                val: 9,
                                next: Option::Some(Box::new(ListNode {
                                    val: 9,
                                    next: Option::Some(Box::new(ListNode {
                                        val: 9,
                                        next: Option::Some(Box::new(ListNode {
                                            val: 9,
                                            next: Option::None
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                })),
                Option::Some(Box::new(ListNode {
                    val: 9,
                    next: Option::Some(Box::new(ListNode {
                        val: 9,
                        next: Option::Some(Box::new(ListNode {
                            val: 9,
                            next: Option::Some(Box::new(ListNode {
                                val: 9,
                                next: Option::None
                            }))
                        }))
                    }))
                }))
            )
        );
    }
}
