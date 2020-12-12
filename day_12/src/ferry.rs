use super::vector::Vector2D;

// All possible actions the ferry can perform
pub enum Action {
    Invalid,
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32)
}

impl Action {
    // Convert a line with the format <action><value> into an action
    fn from_line(line: &str) -> Self {
        let action = line.chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match action {
            'N' => Action::North(value),
            'S' => Action::South(value),
            'E' => Action::East(value),
            'W' => Action::West(value),
            'L' => Action::Left(value),
            'R' => Action::Right(value),
            'F' => Action::Forward(value),
            _ => Action::Invalid
        }
    }
}

// A ferry is an object that can move around and determine its position by reading a list of actions
pub struct Ferry {
    actions: Vec<Action>,
    current_position: Vector2D,
    current_direction: Vector2D,
    next_action_index: usize
}

impl Ferry {
    // Create a new ferry from a list of input actions (puzzle input)
    pub fn new(input_as_strings: &Vec<String>) -> Self {
        // Parse puzzle input into a list of actions
        let mut actions = Vec::new();
        for line in input_as_strings {
            actions.push(Action::from_line(line));
        }

        assert_eq!(actions.len() > 0, true, "Input needs to contain at least one action");

        // According to the puzzle input, the ferry starts by facing east
        //
        //         Y+1
        //          N
        //          |
        //  X-1 W---+---E X+1
        //          |
        //          S
        //         Y-1
        //
        // The compass is represented by a 2D vector, whose axes are mapped as shown above
        let east = Vector2D { x: 1, y: 0 };

        // Construct a ferry with all known data
        Ferry { actions: actions, current_position: Vector2D::new(), current_direction: east, next_action_index: 0 }
    }

    // Move the ferry by executing the next action in the list
    // Actions are interpreted as described here: https://adventofcode.com/2020/day/12#part1
    // Returns true as long as an action was executed, false when no more actions are available
    pub fn execute_next_action_abs(&mut self) -> bool {
        // Last action has already been exectuted
        if self.next_action_index >= self.actions.len() {
            return false;
        }

        let action = &self.actions[self.next_action_index];
        self.next_action_index += 1;

        // Execute the action
        match action {
            Action::North(value) => self.current_position.y += *value,
            Action::South(value) => self.current_position.y -= *value,
            Action::East(value) => self.current_position.x += *value,
            Action::West(value) => self.current_position.x -= *value,
            Action::Left(value) => self.current_direction.rotate(*value),
            Action::Right(value) => self.current_direction.rotate(-*value),
            Action::Forward(value) => {
                // Multiply the direction with the forward distance to get an offset into the direction the ferry is facing
                let new_position = self.current_direction * Vector2D { x: *value, y: *value };

                // Sum the new position and our current position to get the world-space position of the ferry
                self.current_position = self.current_position + new_position;
            },
            Action::Invalid => ()
        }

        // Action executed successfully
        true
    }

    // Calculate the Manhattan distance from (0, 0) to the current position of the ferry
    pub fn get_distance_from_start(&self) -> u32 {
        Vector2D::manhattan_distance(&Vector2D::new(), &self.current_position)
    }
}
