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

        let answer = earliest_bus.unwrap().0 * earliest_bus.unwrap().1;
        println!("Answer part one: {} is the ID of the earliest bus to the airport, multiplied with the waiting time", answer);
    }

    // Part two: what is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?
    fn solve_part_two(&self) {
        // Only need the second line with bus IDs
        let bus_ids = self.input[1].split(',').collect::<Vec<_>>();

        let mut previous_timestamp = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();

        // The puzzle says that the timestamp surely will be larger than 100000000000000
        // The timestamp starts at that number to save some time when brute-forcing the solution
        let mut timestamp = 100_000_000_000_000u64;
        loop {
            let mut first_departure_time = 0;
            let mut all_good = true;

            for (index, id) in bus_ids.iter().enumerate() {
                // Found an 'x' in the input string
                if id.parse::<u64>().is_err() {
                    continue;
                }

                let bus_id = id.parse::<u64>().unwrap();

                // Calculate next departure time as we did in the first solution
                let route_process = timestamp % bus_id;
                let time_left = bus_id - route_process;
                let next_departure = timestamp + time_left;

                // Save first departure time of the sequence
                if index == 0 {
                    first_departure_time = next_departure;
                }

                // Bus 'bus_id' needs to depart at timestamp + 'index'
                if next_departure != first_departure_time as u64 + index as u64 {
                    all_good = false;
                }
            }

            // Found the timestamp at which all listed busses depart at offsets matching their positions in the list
            if all_good {
                println!("Answer part two: {} is the first timestamp at which all listed busses depart at offsets matching their positions in the list", first_departure_time);
                return;
            }

            // Every 5M iterations, log the progress to show that the application is still running
            if timestamp % 5_000_000 == 0 {
                let current_timestamp = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();
                println!("{}\tStill searching... timestamp: {}\ttook: {} seconds", current_timestamp, timestamp, current_timestamp - previous_timestamp);
                previous_timestamp = current_timestamp;
            }

            // Brute force the next timestamp until a solution has been found
            timestamp += 1;
        }
    }
}
