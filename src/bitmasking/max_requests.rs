use std::collections::{HashSet, HashMap};

/// The following code failed some tests
pub fn find_loop(graph: &Vec<HashMap<usize, u32>>, start: usize) -> Option<Vec<usize>> {
    let mut explored = HashSet::new();
    let mut path = Vec::new();

    find_loop_recursively(graph, start, &mut explored, &mut path)
        .map(move |_| {
            let last = path[path.len() - 1];

            path.iter()
                .copied()
                .skip_while(|&building| building != last)
                .collect()
        })
}

pub fn find_loop_recursively(graph: &Vec<HashMap<usize, u32>>, current: usize, explored: &mut HashSet<usize>, path: &mut Vec<usize>) -> Option<()> {
    path.push(current);

    if explored.contains(&current) {
        return Some(());
    }

    explored.insert(current);

    if graph[current].len() == 0 {
        return None;
    }

    for &link in graph[current].keys() {
        if let Some(_) = find_loop_recursively(graph, link, explored, path) {
            return Some(());
        }

        path.pop();
        explored.remove(&current);
    }

    None
}

pub fn maximum_requests2(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    let mut graph: Vec<HashMap<usize, u32>> = vec![HashMap::new(); n as usize];
    let mut reqs: Vec<(usize, usize)> = Vec::new();
    let mut accepted_count = 0;

    for request in requests {
        let src = request[0] as usize;
        let dst = request[1] as usize;

        if src == dst {
            accepted_count += 1;
        } else {
            *graph[src].entry(dst).or_insert(0) += 1;
            reqs.push((src, dst));
        }
    }

    println!("Requests = {reqs:?}");
    println!("Graph = {graph:?}");

    for (src, dst) in reqs {
        if graph[src].contains_key(&dst) {
            if let Some(path) = find_loop(&graph, src) {
                println!("Loop found = {path:?}");
                accepted_count += path.len() - 1;

                for request in path.windows(2) {
                    let (src, dst) = (request[0], request[1]);
                    println!("Request = {request:?}");
                    let req_count = graph[src].get_mut(&dst).unwrap();
                    *req_count -= 1;

                    if *req_count == 0 {
                        graph[src].remove(&dst);
                    }
                }
            }
        }
    }

    accepted_count as i32
}

/// Everything below this point passes all tests

const MAX_REQUESTS: usize = 16;

pub fn accepted_requests(n: usize, bitmask: usize, requests: &Vec<Vec<i32>>) -> Option<i32> {
    let mut net_change = vec![0; n];
    let mut accepted_count = 0;

    for (i, request) in requests.into_iter().enumerate() {
        if bitmask & (1 << i) > 0 {
            net_change[request[0] as usize] -= 1;
            net_change[request[1] as usize] += 1;
            accepted_count += 1;
        }
    }

    net_change
        .into_iter()
        .all(|it| it == 0)
        .then_some(accepted_count)
}

pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    (0..1 << MAX_REQUESTS)
        .filter_map(|bitmask| accepted_requests(n as usize, bitmask, &requests))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::maximum_requests;

    #[test]
    fn test1() {
        let requests = vec![vec![0,1],vec![1,0],vec![0,1],vec![1,2],vec![2,0],vec![3,4]];
        let result = maximum_requests(5, requests);
        assert_eq!(result, 5);
    }

    #[test]
    fn test2() {
        let requests = vec![vec![0,0],vec![1,2],vec![2,1]];
        let result = maximum_requests(3, requests);
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let requests = vec![vec![0,3],vec![3,1],vec![1,2],vec![2,0]];
        let result = maximum_requests(4, requests);
        assert_eq!(result, 4);
    }

    #[test]
    fn test4() {
        let requests = vec![vec![1,2],vec![1,2],vec![2,2],vec![0,2],vec![2,1],vec![1,1],vec![1,2]];
        let result = maximum_requests(3, requests);
        assert_eq!(result, 4);
    }

    #[test]
    fn test5() {
        let requests = vec![vec![0,0],vec![1,1],vec![0,0],vec![2,0],vec![2,2],vec![1,1],vec![2,1],vec![0,1],vec![0,1]];
        let result = maximum_requests(3, requests);
        assert_eq!(result, 5);
    }

    #[test]
    fn test6() {
        let requests = vec![vec![0,3],vec![3,3],vec![3,1],vec![0,1],vec![3,2],vec![2,2],vec![2,0],vec![1,0],vec![1,0],vec![1,2],vec![2,0],vec![1,3],vec![3,0]];
        let result = maximum_requests(4, requests);
        assert_eq!(result, 10);
    }
}
