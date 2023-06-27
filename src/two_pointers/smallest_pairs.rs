use std::{collections::{BinaryHeap, HashSet}, cmp::Reverse};

/// A pair of values taken from 2 arrays
#[derive(PartialEq, Eq)]
struct Pair {
    /// The pair of values
    pub pair: (i32, i32),
    /// Index of each value in the arrays
    pub indexes: (usize, usize)
}

impl Pair {
    /// Creates a new pair of values
    pub fn new(pair: (i32, i32), indexes: (usize, usize)) -> Self {
        Self {
            pair,
            indexes
        }
    }

    /// Converts the pair of values to a vec (Why leetcode?).
    pub fn to_vec(self) -> Vec<i32> {
        vec![self.pair.0, self.pair.1]
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.pair.0 + self.pair.1).partial_cmp(&(other.pair.0 + other.pair.1))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.pair.0 + self.pair.1).cmp(&(other.pair.0 + other.pair.1))
    }
}

/// You are given two integer arrays nums1 and nums2 sorted in ascending order and an integer k.
/// 
/// Define a pair (u, v) which consists of one element from the first array and one element from the second array.
/// 
/// Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.
pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let mut current = 0;
    let mut explored = HashSet::new();
    let start = Pair::new((nums1[0], nums2[0]), (0, 0));
    let mut heap = BinaryHeap::from([Reverse(start)]);
    let mut pairs = Vec::with_capacity(k as usize);
    
    while let Some(Reverse(pair)) = heap.pop() {
        let (index1, index2) = pair.indexes;
        current += 1;

        let next1 = (index1 + 1, index2);
        let next2 = (index1, index2 + 1);

        if next1.0 < nums1.len() && !explored.contains(&next1) {
            let next = Pair::new((nums1[next1.0], nums2[next1.1]), next1);
            heap.push(Reverse(next));
            explored.insert(next1);
        }

        if next2.1 < nums2.len() && !explored.contains(&next2){
            let next = Pair::new((nums1[next2.0], nums2[next2.1]), next2);
            heap.push(Reverse(next));
            explored.insert(next2);
        }

        pairs.push(pair.to_vec());

        if current == k {
            break;
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::k_smallest_pairs;

    #[test]
    fn test1() {
        let result = k_smallest_pairs(
            vec![1,7,11],
            vec![2,4,6],
            4
        );

        assert_eq!(result, vec![[1,2],[1,4],[1,6]]);
    }
    
    #[test]
    fn test2() {
        let result = k_smallest_pairs(
            vec![1,1,2],
            vec![1,2,3],
            2
        );

        assert_eq!(result, vec![[1,1],[1,1]]);
    }

    #[test]
    fn test3() {
        let result = k_smallest_pairs(
            vec![1,2],
            vec![3],
            3
        );

        assert_eq!(result, vec![[1,3],[2,3]]);
    }

    #[test]
    fn test4() {
        let result = k_smallest_pairs(
            vec![1,2,4,5,6],
            vec![3,5,7,9],
            3
        );

        assert_eq!(result, vec![[1,3],[2,3],[1,5]]);
    }
}
