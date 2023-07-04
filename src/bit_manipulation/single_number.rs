/// Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.
/// 
/// You must implement a solution with a linear runtime complexity and use only constant extra space.
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut once = 0;
    let mut twice = 0;

    for num in nums {
        once = (once ^ num) & !twice;
        twice = (twice ^ num) & !once;
    }

    once
}
