use std::{collections::{HashSet, BinaryHeap, HashMap}, cmp::Ordering, ops::{Add, Index}, fmt::Display};

/// A Cell in a grid or something
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Start,
    Key(u8),
    Lock(u8)
}

/// A tree which contains all posible branching traversals of a set of keys and estimates of their
/// costs
type EstimateTree = HashMap<Point, EstimateNode>;

#[derive(Debug)]
struct EstimateNode {
    total_estimate: u8,
    estimate: u8,
    next: EstimateTree
}

impl EstimateNode {
    pub fn new(estimate: u8, total_estimate: u8, next: EstimateTree) -> Self {
        Self {
            total_estimate,
            estimate,
            next
        }
    }

    /// Builds an estimate tree
    pub fn tree(start: Point, keys: &Vec<Point>) -> EstimateTree {
        let mut tree = HashMap::new();
        let mut explored = [false; 6];

        for i in 0..keys.len() {
            explored[i] = true;
            let key = keys[i];
            let estimate = key.manhattan(start);
            let (total_estimate, subtree) = Self::tree_recurse(i, keys, &mut explored);
            let node = EstimateNode::new(estimate, total_estimate + estimate, subtree);
            explored[i] = false;
            tree.insert(key, node);
        }

        tree
    }

    /// Builds an estimate tree
    fn tree_recurse(current: usize, keys: &Vec<Point>, explored: &mut [bool; 6]) -> (u8, EstimateTree) {
        let mut tree = HashMap::new();
        let key = keys[current];
        let mut cost_estimate = u8::MAX;

        for i in 0..keys.len() {
            if !explored[i] {
                explored[i] = true;
                let next = keys[i];
                let estimate = key.manhattan(next);
                let (total_estimate, subtree) = Self::tree_recurse(i, keys, explored);
                let node = EstimateNode::new(estimate, total_estimate + estimate, subtree);
                cost_estimate = cost_estimate.min(node.total_estimate);
                explored[i] = false;
                tree.insert(next, node);
            }
        }

        if cost_estimate == u8::MAX {
            cost_estimate = 0;
        }

        (cost_estimate, tree)
    }
}

/// An actually usable grid unlike the String vector which leetcode supplies
#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Cell>>,
    keys: Vec<Point>,
    start: Point,
    estimates: EstimateTree
}

impl Grid {
    /// Creates an actually usable grid from a not so usable grid
    pub fn new(old_grid: Vec<String>) -> Self {
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        let mut keys: Vec<Point> = Vec::new();
        let mut start = Point::default();

        for (i, row) in old_grid.into_iter().enumerate() {
            let mut new_row = Vec::new();

            for (j, cell) in row.chars().enumerate() {
                match cell {
                    '.' => new_row.push(Cell::Empty),
                    '#' => new_row.push(Cell::Wall),
                    '@' => {
                        new_row.push(Cell::Start);
                        start = Point(i as u8, j as u8);
                    },
                    'a'..='z' => {
                        let key_id = cell as u8 - b'a';

                        if key_id >= keys.len() as u8 {
                            keys.resize(key_id as usize + 1, Point::default())
                        }

                        keys[key_id as usize] = Point(i as u8, j as u8);
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

            grid.push(new_row);
        }

        let estimates = EstimateNode::tree(start, &keys);

        Self {
            grid,
            keys,
            start,
            estimates
        }
    }

    pub fn rows(&self) -> u8 {
        self.grid.len() as u8
    }

    pub fn cols(&self) -> u8 {
        self.grid[0].len() as u8
    }

    pub fn key_count(&self) -> u8 {
        self.keys.len() as u8
    }
}

impl Index<Point> for Grid {
    type Output = Cell;

    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.0 as usize][index.1 as usize]
    }
}

/// A point in the grid
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy, Default)]
struct Point(u8, u8);

impl Point {
    /// UP, DOWN, LEFT, RIGHT. No need to say anything else.
    const MOVE_DIRECTIONS: [(i8, i8); 4] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1)
    ];

    /// Manhattan distance between 2 points
    pub fn manhattan(self, other: Point) -> u8 {
        ((self.0 as i32 - other.0 as i32).abs() + (self.1 as i32 - other.1 as i32).abs()) as u8
    }

    /// Expands a point into all adjacent points
    pub fn expand(self) -> impl Iterator<Item = Self> {
        Self::MOVE_DIRECTIONS
            .into_iter()
            .filter_map(move |direction| self + direction)
    }
}

impl Add<(i8, i8)> for Point {
    type Output = Option<Self>;

    fn add(self, rhs: (i8, i8)) -> Self::Output {
        let x = self.0 as i8 + rhs.0;
        let y = self.1 as i8 + rhs.1;

        (x > -1 && y > -1)
            .then_some(Point(x as u8, y as u8))
    }
}

/// Traversal state
struct State<'a> {
    cell: Point,
    src: Point,
    dst: Point,
    keys: u8,
    cost: u8,
    estimate: &'a EstimateNode,
    steps: u8
}

impl <'a> State<'a> {
    pub fn new(cell: Point, src: Point, dst: Point, keys: u8, steps: u8, estimate: &'a EstimateNode) -> Self {
        Self {
            cell,
           src,
            dst,
            keys,
            steps,
            estimate,
            cost: steps + ((estimate.total_estimate - estimate.estimate) + cell.manhattan(dst))
        }
    }

    /// Checks if a key has been picked up
    pub fn has_key(&self, key_id: u8) -> bool {
        ((1 << key_id) & self.keys) > 0
    }

    /// Expands a state into all possible neighboring states
    pub fn expand<'b>(self, grid: &'b Grid, exclude: &'b HashSet<Point>) -> Vec<Self> {
        let cell = self.cell;
        let steps = self.steps + 1;

        cell.expand()
            .filter_map(move |cell| {
                match true {
                    _ if cell.0 < grid.rows() as u8 && cell.1 < grid.cols() as u8 => {
                        match grid[cell] {
                            Cell::Wall => None,
                            _ if exclude.contains(&cell) => None,
                            Cell::Lock(key) if !self.has_key(key) => None,
                            _ => Some(State::new(cell, self.src, self.dst, self.keys, steps, self.estimate))
                        }
                    },
                    _ => None
                }
            }).collect()
    }

    /// Counts the number of keys that have been picked up
    pub fn key_count(&self) -> u8 {
        (0..6)
            .map(|i| self.keys & (1 << i))
            .filter(|&has_key| has_key > 0)
            .count() as u8
    }
}

impl <'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.steps == other.steps && self.estimate.total_estimate == other.estimate.total_estimate
    }
}

impl <'a> Eq for State<'a> {}

impl <'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match other.cost.cmp(&self.cost) {
            Ordering::Equal => self.key_count().cmp(&other.key_count()),
            cmp => cmp
        })
    }
}

impl <'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

/// Does some sort of nested A-Star traversal or something like that. Idk, I'm done with this.
fn shortest_path(grid: &Grid) -> i32 {
    let mut explored: HashMap<(Point, Point), HashSet<Point>> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    for i in 0..grid.key_count() as usize {
        let src = grid.keys[i];
        explored.insert((grid.start, src), HashSet::from([grid.start]));
        let initial = State::new(grid.start, grid.start, src, 0, 0, &grid.estimates[&src]);
        heap.push(initial);

        for j in i + 1..grid.key_count() as usize {
            let dst = grid.keys[j];

            explored.insert((src, dst), HashSet::from([src]));
            explored.insert((dst, src), HashSet::from([dst]));
        }
    }

    while let Some(state) = heap.pop() {
        if state.cell == state.dst {
            let id = if let Cell::Key(id) = grid[state.cell] {
                id
            } else {
                unreachable!("What?")
            };

            let keys = state.keys | (1 << id);

            if keys == (1 << grid.keys.len()) - 1 {
                return state.steps as i32;
            }

            for (&dst, estimate) in state.estimate.next.iter() {
                let next = State::new(state.dst, state.dst, dst, keys, state.steps, estimate);
                heap.push(next);
            }
        } else {
            let pair = (state.src, state.dst);

            for neighbor in state.expand(grid, &explored[&pair]) {
                explored.get_mut(&pair)
                    .unwrap()
                    .insert(neighbor.cell);

                heap.push(neighbor);
            }
        }

    }

    -1
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

    // There are around 5 tests which are still failing in Leetcode
    // WHATEVER. IM DONE WITH THIS, THIS PROBLEM IS DRIVING ME TO INSANITY
    shortest_path(&grid)
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

    #[test]
    fn test5() {
        let grid = [
            ".#........",
            "......#..#",
            ".#B#.#..#.",
            "##...D.#..",
            ".#.......#",
            "##.....a..",
            "...C.#...#",
            "A...#.e.E#",
            "c.@..#...d",
            "#..#.#.b.#"
        ].into_iter()
            .map(String::from);

        let result = shortest_path_all_keys(grid.collect());
        assert_eq!(result, 21);
    }

    #[test]
    fn test6() {
        let grid = [
            ".#......###..#.",
            ".###C..#...b...",
            "..#..#.........",
            ".........#.....",
            ".....@#.#......",
            "#.##...#..##...",
            "..d#...a...#...",
            "..###..........",
            "........#....#.",
            "..#.#..#...c#.#",
            "D#..........#.#",
            "............#A.",
            "..#..##...#....",
            "#...#..#..B....",
            ".....##.....#.."
        ].into_iter()
            .map(String::from);

        let result = shortest_path_all_keys(grid.collect());
        assert_eq!(result, 35);
    }

    #[test]
    fn test7() {
        let grid = [
            "..Ff..#..e.#...",
            ".....#.##...#..",
            "....#.#...#....",
            "##.......##...#",
            "...@#.##....#..",
            "#........b.....",
            "..#...#.....##.",
            ".#....#E...#...",
            "......A.#D.#...",
            "...#...#..#....",
            "...a.#B#.......",
            ".......c.....#.",
            "....#...C#...#.",
            "##.#.....d..#..",
            ".#..#......#..."
        ].into_iter()
            .map(String::from);

        let result = shortest_path_all_keys(grid.collect());
        assert_eq!(result, 42);
    }
}
