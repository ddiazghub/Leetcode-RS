use std::{collections::{HashSet, BinaryHeap}, cmp::Ordering};

#[derive(PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Wall,
    Start,
    Key(u8),
    Lock(u8)
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Cell>>,
    keys: Vec<(u8, u8)>,
    start: (u8, u8)
}

impl Grid {
    pub fn new(grid: Vec<String>) -> Self {
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
                        new_grid.start = (i as u8, j as u8);
                    },
                    'a'..='z' => {
                        let key_id = cell as u8 - b'a';

                        if key_id >= new_grid.keys.len() as u8 {
                            new_grid.keys.resize(key_id as usize + 1, (0, 0))
                        }

                        new_grid.keys[key_id as usize] = (i as u8, j as u8);
                        let new_cell = Cell::Key(key_id);
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
}

#[derive(PartialEq, Eq)]
struct GridState {
    mask: u16,
    steps: u8,
    cost: u8,
    depends_on: Option<u8>
}

impl GridState {
    pub fn new(row: u8, col: u8, keys: u8, steps: u8, cost: u8, depends_on: Option<u8>) -> Self {
        let mask = ((row as u16) << 11) + ((col as u16) << 6) + keys as u16;

        Self {
            mask,
            steps,
            cost,
            depends_on
        }
    }

    pub fn row(&self) -> u8 {
        ((0xF800 & self.mask) >> 11) as u8
    }

    pub fn col(&self) -> u8 {
        ((0x7C0 & self.mask) >> 6) as u8
    }

    pub fn cell(&self) -> (u8, u8) {
        (self.row(), self.col())
    }

    pub fn keys(&self) -> u8 {
        (0x3F & self.mask) as u8
    }

    pub fn has_key(&self, key_id: u8) -> bool {
        ((1 << key_id) & self.mask) > 0
    }

    pub fn expand(self, end: (u8, u8), grid: &Grid, exclude: &HashSet<(u8, u8)>) -> Vec<Self> {
        let cell = self.cell();
        let mut keys = self.keys();
        let steps = self.steps + 1;

        if let Cell::Lock(id) = grid.grid[cell.0 as usize][cell.1 as usize] {
            keys |= 1 << id;
        }

        let next = [
            (cell.0 as i8 + 1, cell.1 as i8),
            (cell.0 as i8 - 1, cell.1 as i8),
            (cell.0 as i8, cell.1 as i8 + 1),
            (cell.0 as i8, cell.1 as i8 - 1),
        ];

        next
            .into_iter()
            .filter_map(|(row, col)| {
                match true {
                    _ if (0..grid.grid.len() as i8).contains(&row) && (0..grid.grid[0].len() as i8).contains(&col) => {
                        let cell = (row as u8, col as u8);

                        if exclude.contains(&cell) || grid.grid[row as usize][col as usize] == Cell::Wall {
                            None
                        } else {
                            let cost = steps + manhattan(cell, end);
                            Some(GridState::new(cell.0, cell.1, keys, steps, cost, self.depends_on))
                        }
                    },
                    _ => None
                }
            }).collect()
    }
}

impl PartialOrd for GridState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for GridState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Vec<Edge>>,
    start_edges: Vec<Edge>
}

#[derive(Clone, Copy, Debug)]
enum Edge {
    Indirect(u8),
    Direct {
        cost: u8,
        keys: u8
    }
}

#[derive(PartialEq, Eq)]
struct GraphState {
    current: u8,
    cost: u8,
    keys: u8
}

impl GraphState {
    pub fn new(current: u8, cost: u8, keys: u8) -> Self {
        Self {
            current,
            cost,
            keys
        }
    }

    pub fn key_count(&self) -> u8 {
        (0_u8..6_u8)
            .filter(|&i| ((1 << i) & self.keys) > 0)
            .count() as u8
    }
}

impl PartialOrd for GraphState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.cost.cmp(&self.cost) {
            Ordering::Equal => Some(self.key_count().cmp(&other.key_count())),
            cmp => Some(cmp)
        }
    } 
}

impl Ord for GraphState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn manhattan(point1: (u8, u8), point2: (u8, u8)) -> u8 {
    ((point1.0 as i32 - point2.0 as i32).abs() + (point1.1 as i32 - point2.1 as i32).abs()) as u8
}

fn make_edge(grid: &Grid, src: u8, start: (u8, u8), end: (u8, u8)) -> Option<Edge> {
    let mut explored: HashSet<(u8, u8)> = HashSet::from([start]);
    let initial = GridState::new(start.0, start.1, 0, 0, manhattan(start, end), None);
    let mut heap = BinaryHeap::from([initial]);

    while let Some(mut state) = heap.pop() {
        let cell = state.cell();

        if end == cell {
            return Some(match state.depends_on {
                Some(key) => Edge::Indirect(key),
                None => Edge::Direct {
                    cost: state.steps,
                    keys: state.keys()
                }
            })
        }

        match grid.grid[cell.0 as usize][cell.1 as usize] {
            Cell::Key(id) if id != src => state.depends_on = state.depends_on.is_none().then(|| id),
            _ => ()
        };

        for neighbor in state.expand(end, grid, &explored) {
            explored.insert(neighbor.cell());
            heap.push(neighbor);
        }
    }

    None
}

fn make_graph(grid: &Grid) -> Option<Graph> {
    let n_keys = grid.keys.len();
    let mut edges = vec![vec![Edge::Direct { cost: 0, keys: 0 }; n_keys]; n_keys];
    let mut start_edges = vec![Edge::Indirect(0); n_keys];

    for i in 0..n_keys - 1 {
        let start = grid.keys[i as usize];
        start_edges[i] = make_edge(grid, 255, grid.start, start)?;

        for j in i + 1..n_keys {
            let end = grid.keys[j as usize];
            let edge = make_edge(grid, i as u8, start, end)?;
            edges[i][j] = edge;
            edges[j][i] = edge;
        }
    }

    start_edges[n_keys - 1] = make_edge(grid, 255, grid.start, grid.keys[n_keys - 1])?;

    Some(Graph {
        edges,
        start_edges
    })
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
    let grid = Grid::new(grid);
    
    return match make_graph(&grid) {
        Some(graph) => {
            let target = (1 << grid.keys.len()) as u8 - 1;
            let mut heap = BinaryHeap::new();

            for (i, edge) in graph.start_edges.iter().enumerate() {
                match edge {
                    &Edge::Direct { cost, keys } if keys == 0 => {
                        let state = GraphState::new(i as u8, cost, 1 << i as u8);
                        heap.push(state)
                    },
                    _ => ()
                }
            }

            while let Some(state) = heap.pop() {
                if state.keys == target {
                    return state.cost as i32;
                }

                for (i, edge) in graph.edges[state.current as usize].iter().enumerate() {
                    match edge {
                        &Edge::Direct { cost, keys } if i as u8 != state.current && (state.keys & keys) == keys => {
                            let next = GraphState::new(i as u8, state.cost + cost, state.keys | (1 << i as u8));
                            heap.push(next);
                        },
                        _ => ()
                    }
                }
            }

            -1
        },
        _ => -1
    }
}

#[cfg(test)]
mod tests {
    use super::shortest_path_all_keys;

    #[test]
    fn test1() {
        let grid = ["@.a..","###.#","b.A.B"]
            .into_iter()
            .map(String::from);

        let result = shortest_path_all_keys(grid.collect());
        assert_eq!(result, 8);
    }

    #[test]
    fn test2() {
        let grid = [".@aA"]
            .into_iter()
            .map(String::from);

        let result = shortest_path_all_keys(grid.collect());
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let grid = ["b","A","a","@","B"]
            .into_iter()
            .map(String::from);

        let result = shortest_path_all_keys(grid.collect());
        assert_eq!(result, 3);
    }

    #[test]
    fn test4() {
        let grid = ["@...a",".###A","b.BCc"]
            .into_iter()
            .map(String::from);

        let result = shortest_path_all_keys(grid.collect());
        assert_eq!(result, 10);
    }
}
