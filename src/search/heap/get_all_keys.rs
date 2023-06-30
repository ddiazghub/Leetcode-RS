use std::{collections::{HashSet, BinaryHeap}, borrow::Cow};

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Start,
    Key(u8),
    Lock(u8)
}

struct Grid {
    grid: Vec<Vec<Cell>>,
    keys: Vec<(usize, usize)>,
    start: (usize, usize)
}

#[derive(PartialEq, Eq)]
struct AStarState<'a> {
    cell: (usize, usize),
    keys: Cow<'a, Vec<usize>>,
    steps: u32,
    cost: u32
}

impl <'a> AStarState<'a> {
    pub fn new(cell: (usize, usize), keys: Cow<'a, Vec<usize>>, steps: u32, cost: u32) -> Self {
        Self {
            cell,
            keys,
            steps,
            cost
        }
    }
}

impl PartialOrd for AStarState<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for AStarState<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn parse_grid(grid: Vec<String>) -> Grid {
    let mut new_grid = Grid {
        grid: Vec::new(),
        keys: Vec::new(),
        start: (0, 0)
    };

    for (i, row) in grid.into_iter().enumerate() {
        let mut new_row = Vec::new();

        for (j, cell) in row.chars().enumerate() {
            match cell {
                '.' => new_row.push(Cell::Empty),
                '#' => new_row.push(Cell::Wall),
                '@' => {
                    new_row.push(Cell::Start);
                    new_grid.start = (i, j);
                },
                'a'..='z' => {
                    new_grid.keys.push((i, j));
                    let new_cell = Cell::Key(cell as u8 - b'a');
                    new_row.push(new_cell);
                },
                'A'..='Z' => {
                    let new_cell = Cell::Lock(cell as u8 - b'A');
                    new_row.push(new_cell);
                },
                _ => panic!("What?")
            }
        }

        new_grid.grid.push(new_row);
    }

    new_grid
}

fn manhattan(point1: (usize, usize), point2: (usize, usize)) -> u32 {
    ((point1.0 as i32 - point2.0 as i32) + (point1.1 as i32 - point2.1 as i32)).abs() as u32
}

fn a_star(grid: &Grid, src: usize, dst: usize) -> Option<(u32, Vec<usize>)> {
    let start = grid.keys[src];
    let end = grid.keys[dst];
    let mut explored: HashSet<(usize, usize)> = HashSet::from([start]);
    let mut heap: BinaryHeap<AStarState> = BinaryHeap::new();

    heap.push(AStarState::new(start, Cow::Owned(Vec::new()), 0, manhattan(start, end)));

    while let Some(state) = heap.pop() {
        if end == state.cell {
            return Some((state.steps, state.keys.into_owned()));
        }

        let next = [
            (state.cell.0 + 1, state.cell.1),
            (state.cell.0 - 1, state.cell.1),
            (state.cell.0, state.cell.1 + 1),
            (state.cell.0, state.cell.1 - 1),
        ];

        let steps = state.steps + 1;
        
        /*
        for cell in next {
            if cell.0 < grid.grid.len() &&
                cell.1 < grid.grid[0].len() &&
                !explored.contains(&cell) &&
                grid.grid[cell.0][cell.1] != Cell::Wall
            {
                let keys = if let Cell::Lock(id) = grid.grid[state.cell.0][state.cell.1] {
                    let mut keys = state.keys.into_owned();
                    keys.push(id as usize);
                    Cow::Owned(keys)
                } else {
                    state.keys
                };


                explored.insert(cell);
                let new_state = AStarState::new(cell, keys, steps, steps + manhattan(cell, end));
                heap.push(new_state);
            }
        }
        */
    }

    None
}

fn get_distances(grid: &Grid) -> Vec<Vec<usize>> {
    let n_keys = grid.keys.len();
    let mut distances = vec![vec![0; n_keys]; n_keys];

    for i in 0..n_keys {
        for j in 0..n_keys {
            if i == j {
                continue;
            }

            
        }
    }

    distances
}

/// You are given an m x n grid grid where:
/// 
/// '.' is an empty cell.
/// '#' is a wall.
/// '@' is the starting point.
/// Lowercase letters represent keys.
/// Uppercase letters represent locks.
/// You start at the starting point and one move consists of walking one space in one of the four cardinal directions. You cannot walk outside the grid, or walk into a wall.
/// 
/// If you walk over a key, you can pick it up and you cannot walk over a lock unless you have its corresponding key.
/// 
/// For some 1 <= k <= 6, there is exactly one lowercase and one uppercase letter of the first k letters of the English alphabet in the grid. This means that there is exactly one key for each lock, and one lock for each key; and also that the letters used to represent the keys and locks were chosen in the same order as the English alphabet.
/// 
/// Return the lowest number of moves to acquire all keys. If it is impossible, return -1.
pub fn shortest_path_all_keys(grid: Vec<String>)/* Strings can't be indexed in Rust, really Leetcode? */ -> i32 {
    let grid = parse_grid(grid);

    0
}

#[cfg(test)]
mod tests {
    use super::shortest_path_all_keys;
}
