const CHARS: usize = 26;

/// The variance of a string is defined as the largest difference between the number of occurrences of any 2 characters present in the string. Note the two characters may or may not be the same.
/// 
/// Given a string s consisting of lowercase English letters only, return the largest variance possible among all substrings of s.
/// 
/// A substring is a contiguous sequence of characters within a string.
pub fn largest_variance(s: String) -> i32 {
    let mut char_counts = [0; CHARS];

    for ch in s.bytes() {
        char_counts[(ch - b'a') as usize] += 1;
    }

    let mut max = 0;

    for first in 0..CHARS {
        if char_counts[first] == 0 {
            continue;
        }

        for second in 0..CHARS {
            if first == second || char_counts[second] == 0 {
                continue;
            }

            let mut remaining = char_counts[second];
            let mut counts = (0, 0);
            let chars = (first as u8 + b'a', second as u8 + b'a');

            for ch in s.bytes() {
                match ch {
                    _ if ch == chars.0 => counts.0 += 1,
                    _ if ch == chars.1 => {
                        counts.1 += 1;
                        remaining -= 1;
                    },
                    _ => {}
                };

                let variance = counts.0 - counts.1;

                if counts.1 > 0 {
                    max = max.max(variance);
                }

                if variance < 0 && remaining > 0 {
                    counts = (0, 0);
                }
            }
        }
    }

    max
}
