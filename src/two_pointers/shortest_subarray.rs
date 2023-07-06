/// A window, slice or subarray taken from an array
struct Window {
    /// Start offset of the window in the array
    start: usize,
    /// Ending offset of the window in the array (Exclusive)
    end: usize,
    /// Sum of the elements in the window
    sum: i32
}

impl Window {
    /// Gets the window's length
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

/// Given an array of positive integers nums and a positive integer target, return the minimal length of a subarray
/// whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    // Initialize the window
    let mut window = Window {
        start: 0,
        end: 0,
        sum: 0
    };

    let mut min_len = i32::MAX;

    while min_len > 1 {
        if window.sum < target {
            // If the end of the array has been reached, stop
            if window.end == nums.len() {
                break;
            }

            // If the sum of the elements in the window is smaller than target, add the next
            // element of the array to the window
            window.sum += nums[window.end];
            window.end += 1;
        } else {
            // Otherwise, remove the first element from the window
            min_len = min_len.min(window.len() as i32);
            window.sum -= nums[window.start];
            window.start += 1;
        }
    }

    if min_len == i32::MAX {
        0
    } else {
        min_len
    }
}
