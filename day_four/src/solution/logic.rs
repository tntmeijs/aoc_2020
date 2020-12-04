use shared::file_io::input::read_input_as_vec;
use shared::traits::puzzle_trait::PuzzleTrait;

struct Passport {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: String
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            birth_year:         "".to_string(),
            issue_year:         "".to_string(),
            expiration_year:    "".to_string(),
            height:             "".to_string(),
            hair_color:         "".to_string(),
            eye_color:          "".to_string(),
            passport_id:        "".to_string(),
            country_id:         "".to_string() }
    }

    fn is_valid_ignore_country(&self) -> bool {
        self.birth_year.len()       > 0 &&
        self.issue_year.len()       > 0 &&
        self.expiration_year.len()  > 0 && 
        self.height.len()           > 0 &&
        self.hair_color.len()       > 0 &&
        self.eye_color.len()        > 0 &&
        self.passport_id.len()      > 0
    }

    fn has_country_id(&self) -> bool {
        self.country_id.len() > 0
    }
}

pub struct DayFour {
    passports: Vec<Passport>
}

impl DayFour {
    pub fn new() -> DayFour {
        DayFour { passports: Vec::new() }
    }
}

impl PuzzleTrait for DayFour {
    fn print_info(&self) {
        println!("DAY 4 - PASSPORT PROCESSING")
    }

    fn gather_input(&mut self) {
        let raw_input = read_input_as_vec("./input/day_four.txt");
    }

    // Part one: check how many documents are valid passports
    fn solve_part_one(&self) {
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
