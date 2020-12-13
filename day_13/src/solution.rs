use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day13 {
    input: Vec<String>
}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day13 {
    fn print_info(&self) {
        println!("DAY 13 - SHUTTLE SEARCH");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_13.txt");
    }

    // Part one: what is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you will need to wait for that bus?
    fn solve_part_one(&self) {
        // First line is the earliest possible time we could depart on a bus
        let current_time = self.input[0].parse::<u32>().unwrap();

        // The second line contains a list of bus IDs, each ID indicates the interval at which the bus leaves
        // Some substrings will be 'x' instead of a timestamp, the filter takes care of that
        let busses = self.input[1]
            .split(',')
            .filter(|item| item.parse::<u32>().is_ok())
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        // Pair: wait time, bus ID
        let mut earliest_bus = None;
        
        for bus in busses {
            // The bus ID equals the interval at which it departs, each bus departs every 'ID' minutes
            // Take the modulo of the current time and the bus interval to figure out how long the bus has been driving already
            let route_progress = current_time % bus;

            // Subtract the total route time from the route progress to get the number of minutes left until we could board this bus
            let time_left = bus - route_progress;

            // The calculations above are based on a start time of zero, but we arrive at 'earliest' seconds
            // Add the current time to the time left to get the next timestamp at which the bus departs
            let next_departure = current_time + time_left;

            // Time we have to wait
            let wait_time = next_departure - current_time;

            if earliest_bus.is_none() {
                // First iteration, save the bus id and wait time
                earliest_bus = Some((wait_time, bus));
            } else {
                // Always save the bus that departs the soonest
                if earliest_bus.unwrap().0 > wait_time {
                    earliest_bus = Some((wait_time, bus));
                }
            }
        }

        println!("{:?}", earliest_bus);

        let answer = earliest_bus.unwrap().0 * earliest_bus.unwrap().1;
        println!("Answer part one: {} is the ID of the earliest bus to the airport, multiplied with the waiting time", answer);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        let answer = -1;
        println!("Answer part two: {}", answer);
    }
}
