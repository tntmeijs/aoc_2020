// All states a single cell in the grid can have
#[derive(PartialEq)]
enum CellState {
    Floor,
    Occupied,
    Available
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
        assert_eq!(rows > 1, true);
        assert_eq!(columns > 1, true);

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

    // Number of rows in the grid
    fn row_count(&self) -> usize {
        self.rows
    }

    // Number of columns in the grid
    fn column_count(&self) -> usize {
        self.columns
    }

    // Try to update the value of a cell
    // Returns true if the cell was found, false otherwise
    fn try_set_cell(&mut self, row: usize, column: usize, state: CellState) -> bool {
        if row > self.row_count() || column > self.column_count() {
            return false;
        }

        self.data[row][column] = state;
        true
    }

    // Check if the cell is set to the specified state
    fn cell_equals(&self, row: usize, column: usize, state: CellState) -> bool {
        if row > self.row_count() || column > self.column_count() {
            return false;
        }

        self.data[row][column] == state
    }
}
