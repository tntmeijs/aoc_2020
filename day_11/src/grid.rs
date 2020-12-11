// All states a single cell in the grid can have
#[derive(PartialEq, Clone)]
pub enum CellState {
    Invalid,
    Floor,
    Occupied,
    Available
}

// All possible locations that can surround a cell
pub enum CellNeighbour {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
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
#[derive(Clone)]
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

        for _row in 0..rows {
            let mut row_data = Vec::new();

            for _column in 0..columns {
                row_data.push(CellState::Floor);
            }

            data.push(row_data);
        }

        let rows = data.len();
        let columns = data[0].len();

        Grid { data: data, rows: rows, columns: columns }
    }

    // Check if two grids are equal in size and data
    pub fn are_equal(a: &Grid, b: &Grid) -> bool {
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

    // Update the state of a cell
    pub fn set_cell(&mut self, row: usize, column: usize, state: CellState) {
        if row > self.rows || column > self.columns {
            panic!("Cell ({}, {}) is out of bounds", row, column);
        }

        self.data[row][column] = state;
    }

    // Get the state of a cell
    pub fn get_cell_state(&self, row: usize, column: usize) -> CellState {
        if row > self.rows || column > self.columns {
            panic!("Cell ({}, {}) is out of bounds", row, column);
        }

        self.data[row][column].clone()
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

    // Get the state of one of the eight potential neighbouring cells
    pub fn get_neightbour_state(&self, row: usize, column: usize, neighbour: CellNeighbour) -> CellState {
        let max_rows = self.rows - 1;
        let max_columns = self.columns - 1;

        match neighbour {
            CellNeighbour::Top =>          { if row == 0                                    { CellState::Invalid } else { self.data[row - 1][column].clone() } },
            CellNeighbour::Bottom =>       { if row >= max_rows                             { CellState::Invalid } else { self.data[row + 1][column].clone() } },
            CellNeighbour::Left =>         { if column == 0                                 { CellState::Invalid } else { self.data[row][column - 1].clone() } },
            CellNeighbour::Right =>        { if column >= max_columns                       { CellState::Invalid } else { self.data[row][column + 1].clone() } },
            CellNeighbour::TopLeft =>      { if row == 0 || column == 0                     { CellState::Invalid } else { self.data[row - 1][column - 1].clone() } },
            CellNeighbour::TopRight =>     { if row == 0 || column >= max_columns           { CellState::Invalid } else { self.data[row - 1][column + 1].clone() } },
            CellNeighbour::BottomLeft =>   { if row >= max_rows || column == 0              { CellState::Invalid } else { self.data[row + 1][column - 1].clone() } },
            CellNeighbour::BottomRight =>  { if row >= max_rows || column >= max_columns    { CellState::Invalid } else { self.data[row + 1][column + 1].clone() } }
        }
    }
}
