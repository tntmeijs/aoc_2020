use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day15 {
    input: Vec<u32>
}

impl Day15 {
    pub fn new() -> Day15 {
        Day15 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day15 {
    fn print_info(&self) {
        println!("DAY 15 - RAMBUNCTIOUS RECITATION");
    }

    fn gather_input(&mut self) {
        for part in read_input_as_vec("./input/day_15.txt")[0].split(',').map(|c| c.parse::<u32>().unwrap()).collect::<Vec<_>>() {
            self.input.push(part);
        }
    }

    // Part one: what will be the 2020th number spoken?
    fn solve_part_one(&self) {
        let mut numbers_spoken = Vec::from(self.input.clone());

        let mut turn = numbers_spoken.len();
        while turn < 10 {
            let last_number = numbers_spoken[turn - 1];
            let occurances = numbers_spoken.iter().filter(|&n| *n == last_number).count();

            println!("Turn {}, last spoken: {}, first time? {}", turn + 1, last_number, occurances < 2);

            if occurances < 2 {
                numbers_spoken.push(0);
            } else {
                
            }

            numbers_spoken.push(0);
            turn += 1;
        }

        println!("Answer part one: {} is the 2020th number spoken", numbers_spoken.last().unwrap());
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
