use std::collections::VecDeque;

/// A Window taken from an array, vector or sequence of items
#[derive(Clone)]
struct Window {
    /// Queue which stores the lengths of at most the last <capacity> subarrays with consecutive
    /// elements.
    lens: VecDeque<i32>,
    /// Start of the current subarray
    start: usize,
    /// Total number of elements in the window
    total_len: i32,
    /// Maximum number of subarrays that can be in the window
    capacity: usize
}

impl Window {
    /// Creates a new window
    pub fn new(capacity: usize) -> Self {
        Window {
            lens: VecDeque::new(),
            start: 0,
            total_len: 0,
            capacity
        }
    }

    /// Adds a subarray to the window, removing the first subarray if the number of subarrays in
    /// the window exceeds capacity
    pub fn add(&mut self, length: i32) {
        self.lens.push_back(length);
        self.total_len += length;

        if self.lens.len() > self.capacity {
            self.total_len -= self.lens.pop_front().unwrap_or(0);
        }
    }

    /// Calculates the number of consecutive elmements in the window
    pub fn consecutive(&self) -> i32 {
        self.total_len + self.lens.len() as i32 - 1
    }
}

/// A teacher is writing a test with n true/false questions, with 'T' denoting true and 'F' denoting false. He wants to confuse the students by maximizing the number of consecutive questions with the same answer (multiple trues or multiple falses in a row).
/// 
/// You are given a string answerKey, where answerKey[i] is the original answer to the ith question. In addition, you are given an integer k, the maximum number of times you may perform the following operation:
/// 
/// Change the answer key for any question to 'T' or 'F' (i.e., set answerKey[i] to 'T' or 'F').
/// Return the maximum number of consecutive 'T's or 'F's in the answer key after performing the operation at most k times.
pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let mut true_window = Window::new(k as usize + 1);
    let mut false_window = true_window.clone();
    let mut n = 0;
    let mut max = 0;

    for (i, key) in answer_key.chars().enumerate() {
        n += 1;

        match key {
            'T' => {
                false_window.add((i - false_window.start) as i32);
                max = max.max(false_window.consecutive());
                false_window.start = i + 1;
            },
            'F' => {
                true_window.add((i - true_window.start) as i32);
                max = max.max(true_window.consecutive());
                true_window.start = i + 1;
            },
            _ => panic!("What?")
        }
    }

    false_window.add((n - false_window.start) as i32);
    true_window.add((n - true_window.start) as i32);
    max = max.max(true_window.consecutive().max(false_window.consecutive()));

    max
}
