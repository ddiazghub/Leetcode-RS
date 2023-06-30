use std::{collections::{HashSet, BinaryHeap}, cmp::Ordering};

/// A (row, col) cell in a grid.
#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct Cell(u16, u16);

impl Cell {
    /// Tries to create a cell and returns it if the cell is in the grid, returning None otherwise.
    pub fn try_new(row: i16, col: i16, rows: u16, cols: u16) -> Option<Self> {
        if (0..rows as i16).contains(&row) && (0..cols as i16).contains(&col) {
            Some(Cell(row as u16, col as u16))
        } else {
            None
        }
    }

    /// Expands a cell and returns all adjacent cells which are valid (Are in the grid).
    pub fn expand<'a>(self, rows: u16, cols: u16) -> Vec<Cell> {
        let next = [
            (self.0 as i16 + 1, self.1 as i16),
            (self.0 as i16 - 1, self.1 as i16),
            (self.0 as i16, self.1 as i16 + 1),
            (self.0 as i16, self.1 as i16 - 1)
        ];

        next
            .into_iter()
            .filter_map(move |(row, col)| Cell::try_new(row, col, rows, cols))
            .collect()
    }
}

/// A state of a grid traversal.
/// Contains the current position in the grid and the last day this
/// position can be reached from any cell in the first row of the grid.
#[derive(PartialEq, Eq)]
struct State {
    /// Current position of the traversal in the grid.
    cell: Cell,
    /// Last day this position can be reached from any cell in the first row of the grid.
    limit: u16
}

impl State {
    /// Creates a new state with given data.
    pub fn new(cell: Cell, limit: u16) -> Self {
        Self {
            cell,
            limit
        }
    }
}

/// States will be ordered based on the last day their position can be reached.
/// If 2 states have the same limit, then the one closest to the goal (Last row of the grid) will
/// be given more priority.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.limit.cmp(&other.limit) {
            Ordering::Equal => Some(self.cell.0.cmp(&other.cell.0)),
            cmp => Some(cmp)
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

/// There is a 1-based binary matrix where 0 represents land and 1 represents water. You are given integers row and col representing the number of rows and columns in the matrix, respectively.
/// 
/// Initially on day 0, the entire matrix is land. However, each day a new cell becomes flooded with water. You are given a 1-based 2D array cells, where cells[i] = [ri, ci] represents that on the ith day, the cell on the rith row and cith column (1-based coordinates) will be covered with water (i.e., changed to 1).
/// 
/// You want to find the last day that it is possible to walk from the top to the bottom by only walking on land cells. You can start from any cell in the top row and end at any cell in the bottom row. You can only travel in the four cardinal directions (left, right, up, and down).
/// 
/// Return the last day where it is possible to walk from the top to the bottom by only walking on land cells.
pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    let rows = row as u16;
    let cols = col as u16;

    // Initialize the grid
    let mut grid = vec![vec![0_u32; cols as usize]; rows as usize];

    // Fill each cell in the grid with the last day that cell can be reached (It's index in the
    // cells vector).
    for (i, cell) in cells.into_iter().enumerate() {
        grid[(cell[0] - 1) as usize][(cell[1] - 1) as usize] = i as u32;
    }

    let get = |row: u16, col: u16| grid[row as usize][col as usize] as u16;
    let iter = (0_u16..cols as u16).map(|i| Cell(0_u16, i));

    // Create a Hashset to store explored cells.
    let mut explored: HashSet<Cell> = iter.clone().collect();

    // A priority queue will be used as a frontier to traverse the grid with a best-first approach.
    // The path with the furthest expiration date will be traversed first.
    let mut heap: BinaryHeap<State> = iter.map(|cell|
        State::new(
            cell,
            get(cell.0, cell.1)
        )
    ).collect();

    // Traversal
    while let Some(state) = heap.pop() {
        // If the current cell is in the last row the goal has been reached.
        // In this case just return the last day this state can be reached.
        if state.cell.0 == rows - 1 {
            return state.limit as i32;
        }

        // Expand the current cell and get all valid neighboring cells.
        for cell in state.cell.expand(rows, cols) {
            // If the cell has already been explored, ignore it.
            if explored.contains(&cell) {
                continue;
            }

            // Add the adjacent cell to the explored set.
            explored.insert(cell);

            // Calculate the last day the adjacent cell can be reached.
            // This is just the smallest value between the last day the current state can be reached
            // and the last day the adjacent cell can be reached.
            //
            // After all, the last day a path in the grid can be traversed is always going to be
            // the smallest of the last days in all cells of that path.
            let limit = state.limit.min(get(cell.0, cell.1));

            // Create a new state based on the adjacent cell and add it to the frontier.
            heap.push(State::new(cell, limit));
        }
    }

    // This should never be reached
    0
}

#[cfg(test)]
mod tests {
    use super::latest_day_to_cross;

    #[test]
    fn test1() {
        let cells = vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]];
        let result = latest_day_to_cross(2, 2, cells);
        assert_eq!(result, 2);
    }
}
