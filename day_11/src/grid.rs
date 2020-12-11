// All states a single cell in the grid can have
#[derive(PartialEq)]
pub enum CellState {
    Invalid,
    Floor,
    Occupied,
    Available
}

impl CellState {
    pub fn from_char(c: char) -> CellState {
        match c {
            '.' => CellState::Floor,
            'L' => CellState::Available,
            '#' => CellState::Occupied,
            _ => CellState::Invalid
        }
    }
}

// Grid of cells
pub struct Grid {
    rows: usize,
    columns: usize,
    data: Vec<Vec<CellState>>
}

impl Grid {
    // Create a new grid with a predefined size
    // All cells will be floors by default
    pub fn new(rows: usize, columns: usize) -> Grid {
        assert_eq!(rows > 1, true, "Grid must have at least 2 rows");
        assert_eq!(columns > 1, true, "Grid must have at least 2 columns");

        let mut data = Vec::new();

        for row in 0..rows {
            let mut row_data = Vec::new();

            for column in 0..columns {
                row_data.push(CellState::Floor);
            }

            data.push(row_data);
        }

        let rows = data.len();
        let columns = data[0].len();

        Grid { data: data, rows: rows, columns: columns }
    }

    // Check if two grids are equal in size and data
    pub fn are_equal(a: Grid, b: Grid) -> bool {
        // Different dimensions
        if a.rows != b.rows || a.columns != b.columns {
            return false;
        }

        for row in 0..a.rows {
            for column in 0..a.columns {
                let cell_a = &a.data[row][column];
                let cell_b = &b.data[row][column];

                // Cells have a different state
                if cell_a != cell_b {
                    return false;
                }
            }
        }

        // Grids have the same size and data
        true
    }

    // Number of rows in the grid
    pub fn row_count(&self) -> usize {
        self.rows
    }

    // Number of columns in the grid
    pub fn column_count(&self) -> usize {
        self.columns
    }

    // Try to update the value of a cell
    // Returns true if the cell was found, false otherwise
    pub fn try_set_cell(&mut self, row: usize, column: usize, state: CellState) -> bool {
        if row > self.rows || column > self.columns {
            return false;
        }

        self.data[row][column] = state;
        true
    }

    // Check if the cell is set to the specified state
    pub fn cell_equals(&self, row: usize, column: usize, state: CellState) -> bool {
        if row > self.rows || column > self.columns {
            return false;
        }

        self.data[row][column] == state
    }

    // Check if any cells are invalid
    pub fn is_valid(&self) -> bool {
        for row in 0..self.rows {
            for column in 0..self.columns {
                if self.data[row][column] == CellState::Invalid {
                    return false;
                }
            }
        }

        // No invalid cells found
        true
    }
}
