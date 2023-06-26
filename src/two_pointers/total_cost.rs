use std::{collections::BinaryHeap, cmp::Reverse};

/// You are given a 0-indexed integer array costs where costs[i] is the cost of hiring the ith worker.
/// 
/// You are also given two integers k and candidates. We want to hire exactly k workers according to the following rules:
/// 
/// You will run k sessions and hire exactly one worker in each session.
/// In each hiring session, choose the worker with the lowest cost from either the first candidates workers or the last candidates workers. Break the tie by the smallest index.
/// For example, if costs = [3,2,7,7,1,2] and candidates = 2, then in the first hiring session, we will choose the 4th worker because they have the lowest cost [3,2,7,7,1,2].
/// In the second hiring session, we will choose 1st worker because they have the same lowest cost as 4th worker but they have the smallest index [3,2,7,7,2]. Please note that the indexing may be changed in the process.
/// If there are fewer than candidates workers remaining, choose the worker with the lowest cost among them. Break the tie by the smallest index.
/// A worker can only be chosen once.
/// Return the total cost to hire exactly k workers.
pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let n = costs.len() as i32;
    let half = (n + 1) / 2;
    let mut costs = costs;

    if candidates >= half {
        costs.sort_unstable();

        return costs
            .into_iter()
            .take(k as usize)
            .map(i64::from)
            .sum();
    }

    let candidates = candidates.min(half);
    let mut k = k;

    if n == k && candidates == half {
        return costs
            .into_iter()
            .map(i64::from)
            .sum()
    }

    let mut low: BinaryHeap<_> = costs
        .iter()
        .take(candidates as usize)
        .map(|it| Reverse(*it))
        .collect();

    let mut high: BinaryHeap<_> = costs
        .iter()
        .rev()
        .take(candidates as usize)
        .map(|it| Reverse(*it))
        .collect();

    let mut current_low = candidates as usize;
    let mut current_high = (n - candidates - 1) as usize;
    let mut total = 0_i64;

    while k > 0 {
        let high_value = high.peek().map_or(i32::MAX, |it| it.0);
        let low_value = low.peek().map_or(i32::MAX, |it| it.0);

        if high_value < low_value {
            high.pop();
            println!("{high_value}");
            total += high_value as i64;

            if current_low <= current_high {
                high.push(Reverse(costs[current_high]));
                current_high -= 1;
            }
        } else {
            low.pop();
            println!("{low_value}");
            total += low_value as i64;

            if current_low <= current_high {
                low.push(Reverse(costs[current_low]));
                current_low += 1;
            }
        };

        k -= 1;
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::total_cost;

    #[test]
    fn total_cost1() {
        let costs = vec![17,12,10,2,7,2,11,20,8];
        let result = total_cost(costs, 3, 4);
        assert_eq!(result, 11);
    }
    
    #[test]
    fn total_cost2() {
        let costs = vec![28,35,21,13,21,72,35,52,74,92,25,65,77,1,73,32,43,68,8,100,84,80,14,88,42,53,98,69,64,40,60,23,99,83,5,21,76,34];
        let result = total_cost(costs, 32, 12);
        assert_eq!(result, 1407);
    }

    #[test]
    fn total_cost3() {
        let costs = vec![18,64,12,21,21,78,36,58,88,58,99,26,92,91,53,10,24,25,20,92,73,63,51,65,87,6,17,32,14,42,46,65,43,9,75];
        let result = total_cost(costs, 13, 23);
        assert_eq!(result, 223);
    }
}
