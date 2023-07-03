use std::collections::{HashSet, BinaryHeap}; 

/// A state during the best-first traversal of an undirected graph
#[derive(PartialEq)]
struct State {
    /// Current position in the graph
    node: usize,
    /// Cumulative probability of the traversal
    probability: f64
}

impl State {
    /// Creates a new state
    pub fn new(node: usize, probability: f64) -> Self {
        Self {
            node,
            probability
        }
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.probability.partial_cmp(&other.probability)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// You are given an undirected weighted graph of n nodes (0-indexed), represented by an edge list where edges[i] = [a, b] is an undirected edge connecting the nodes a and b with a probability of success of traversing that edge succProb[i].
/// 
/// Given two nodes start and end, find the path with the maximum probability of success to go from start to end and return its success probability.
/// 
/// If there is no path from start to end, return 0. Your answer will be accepted if it differs from the correct answer by at most 1e-5.
pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
    let mut graph_edges: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n as usize];

    for (edge, probability) in edges.into_iter().zip(succ_prob.into_iter()) {
        graph_edges[edge[0] as usize].push((edge[1] as usize, probability));
        graph_edges[edge[1] as usize].push((edge[0] as usize, probability));
    }

    let end = end as usize;
    let mut explored: HashSet<usize> = HashSet::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::from([State::new(start as usize, 1.0)]);

    while let Some(state) = heap.pop() {
        if state.node == end {
            return state.probability;
        }

        if explored.contains(&state.node) {
            continue;
        }

        for &(next, probability) in graph_edges[state.node].iter() {
            let new_state = State::new(next, state.probability * probability);
            heap.push(new_state);
        }

        explored.insert(state.node);
    }

    0.0
}

#[cfg(test)]
mod tests {
    use super::max_probability;

    #[test]
    fn test1() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5,0.5,0.2];
        let result = max_probability(3, edges, succ_prob, 0, 2);
        assert_eq!(result, 0.25);
    }

    #[test]
    fn test2() {
        let edges = vec![vec![0,1],vec![1,2],vec![0,2]];
        let succ_prob = vec![0.5,0.5,0.3];
        let result = max_probability(3, edges, succ_prob, 0, 2);
        assert_eq!(result, 0.3);
    }

    #[test]
    fn test3() {
        let edges = vec![vec![0,1]];
        let succ_prob = vec![0.5];
        let result = max_probability(3, edges, succ_prob, 0, 2);
        assert_eq!(result, 0.0);
    }
}
