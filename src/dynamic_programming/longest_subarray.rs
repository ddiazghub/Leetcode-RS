/// Given a binary array nums, you should delete one element from it.
/// 
/// Return the size of the longest non-empty subarray containing only 1's in the resulting array. Return 0 if there is no such subarray.
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut max_len = i32::MIN;
    let mut last = 0;
    let mut current = 0;

    for num in nums {
        match num {
            1 => current += 1,
            _ => {
                max_len = max_len.max(last + current);
                last = current;
                current = 0;
            }
        }
    }


    if max_len == i32::MIN {
        n as i32 - 1
    } else {
        max_len.max(last + current)
    }
}
