/// You are climbing a staircase. It takes n steps to reach the top.
///
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
pub fn climb_stairs(n: i32) -> i32 {
    let mut taking = 1;
    let mut skipping = 0;

    for _ in 0..n {
        let next = taking + skipping;
        skipping = taking;
        taking = next;
    }

    return taking;
}

/// You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.
/// 
/// You can either start from the step with index 0, or the step with index 1.
/// 
/// Return the minimum cost to reach the top of the floor.
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let steps = cost.len();
    let mut memory = [cost[steps - 1], cost[steps - 2]];

    for i in (0..steps - 2).rev() {
        let current = cost[i] + memory[0].min(memory[1]);
        memory = [memory[1], current];
    }

    return memory[0].min(memory[1]);
}
