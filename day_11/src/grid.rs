// All states a single cell in the grid can have
#[derive(PartialEq, Clone)]
pub enum SeatState {
    Invalid,
    Floor,
    Occupied,
    Available
}

// All possible locations that can surround a cell
pub enum GridNeighbour {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

impl SeatState {
    pub fn from_char(c: char) -> SeatState {
        match c {
            '.' => SeatState::Floor,
            'L' => SeatState::Available,
            '#' => SeatState::Occupied,
            _ => SeatState::Invalid
        }
    }
}

// Grid of cells
#[derive(Clone)]
pub struct Grid {
    rows: usize,
    columns: usize,
    data: Vec<Vec<SeatState>>
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
                row_data.push(SeatState::Floor);
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
    pub fn set_seat(&mut self, row: usize, column: usize, state: SeatState) {
        if row > self.rows || column > self.columns {
            panic!("Cell ({}, {}) is out of bounds", row, column);
        }

        self.data[row][column] = state;
    }

    // Get the state of a cell
    pub fn get_cell_state(&self, row: usize, column: usize) -> SeatState {
        if row > self.rows || column > self.columns {
            panic!("Cell ({}, {}) is out of bounds", row, column);
        }

        self.data[row][column].clone()
    }

    // Check if any cells are invalid
    pub fn is_valid(&self) -> bool {
        for row in 0..self.rows {
            for column in 0..self.columns {
                if self.data[row][column] == SeatState::Invalid {
                    return false;
                }
            }
        }

        // No invalid cells found
        true
    }

    // Get the state of one of the eight potential neighbouring cells
    pub fn get_neightbour_state(&self, row: usize, column: usize, neighbour: GridNeighbour) -> SeatState {
        let max_rows = self.rows - 1;
        let max_columns = self.columns - 1;

        match neighbour {
            GridNeighbour::Top =>          { if row == 0                                    { SeatState::Invalid } else { self.data[row - 1][column].clone() } },
            GridNeighbour::Bottom =>       { if row >= max_rows                             { SeatState::Invalid } else { self.data[row + 1][column].clone() } },
            GridNeighbour::Left =>         { if column == 0                                 { SeatState::Invalid } else { self.data[row][column - 1].clone() } },
            GridNeighbour::Right =>        { if column >= max_columns                       { SeatState::Invalid } else { self.data[row][column + 1].clone() } },
            GridNeighbour::TopLeft =>      { if row == 0 || column == 0                     { SeatState::Invalid } else { self.data[row - 1][column - 1].clone() } },
            GridNeighbour::TopRight =>     { if row == 0 || column >= max_columns           { SeatState::Invalid } else { self.data[row - 1][column + 1].clone() } },
            GridNeighbour::BottomLeft =>   { if row >= max_rows || column == 0              { SeatState::Invalid } else { self.data[row + 1][column - 1].clone() } },
            GridNeighbour::BottomRight =>  { if row >= max_rows || column >= max_columns    { SeatState::Invalid } else { self.data[row + 1][column + 1].clone() } }
        }
    }

    // Cast an imaginary ray across the grid to check which seat is visible
    // The closest seat is returned
    pub fn get_visible_seat(&self, row: usize, column: usize, delta_row: i32, delta_column: i32) -> SeatState {
        let max_rows = self.rows - 1;
        let max_columns = self.columns - 1;

        let mut current_row = row as i32;
        let mut current_column = column as i32;

        loop {
            // Check the next cells
            current_row += delta_row;
            current_column += delta_column;

            // Reached the edge of the grid
            if current_row < 0 || current_row > max_rows as i32 || current_column < 0 || current_column > max_columns as i32 {
                break;
            }

            // Look for available or occupied seats
            let seat = &self.data[current_row as usize][current_column as usize];
            if *seat == SeatState::Occupied || *seat == SeatState::Available {
                return seat.clone();
            }
        }

        // Unable to find a seat
        SeatState::Invalid
    }
}
