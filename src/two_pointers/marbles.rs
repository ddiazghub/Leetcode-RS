/// Sum the bags - 1 first items of an i32 iterator into an i64
fn sum(iter: impl Iterator<Item = i32>, bags: usize) -> i64 {
    iter
        .take(bags - 1)
        .map(i64::from)
        .sum()
}

/// You have k bags. You are given a 0-indexed integer array weights where weights[i] is the weight of the ith marble. You are also given the integer k.
/// 
/// Divide the marbles into the k bags according to the following rules:
/// 
/// No bag is empty.
/// If the ith marble and jth marble are in a bag, then all marbles with an index between the ith and jth indices should also be in that same bag.
/// If a bag consists of all the marbles with an index from i to j inclusively, then the cost of the bag is weights[i] + weights[j].
/// The score after distributing the marbles is the sum of the costs of all the k bags.
/// 
/// Return the difference between the maximum and minimum scores among marble distributions.
pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let mut split_sums: Vec<_> = weights.windows(2)
        .map(|window| window[0] + window[1])
        .collect();
        
    split_sums.sort_unstable();

    let min_score = sum(split_sums.iter().copied(), k as usize);
    let max_score = sum(split_sums.into_iter().rev(), k as usize);

    max_score - min_score
}
