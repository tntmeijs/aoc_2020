use shared::file_io::input::read_input_as_vec;
use shared::traits::puzzle_trait::PuzzleTrait;

#[derive(Debug)]
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
    fn is_valid_ignore_country(&self) -> bool {
        !self.birth_year.is_empty()         &&
        !self.issue_year.is_empty()         &&
        !self.expiration_year.is_empty()    &&
        !self.height.is_empty()             &&
        !self.hair_color.is_empty()         &&
        !self.eye_color.is_empty()          &&
        !self.passport_id.is_empty()
    }

    fn has_country_id(&self) -> bool {
        !self.country_id.is_empty()
    }
}

pub struct DayFour {
    valid_passports: Vec<Passport>,
    invalid_passports: Vec<Passport>
}

impl DayFour {
    pub fn new() -> DayFour {
        DayFour { valid_passports: Vec::new(), invalid_passports: Vec::new() }
    }
}

impl PuzzleTrait for DayFour {
    fn print_info(&self) {
        println!("DAY 4 - PASSPORT PROCESSING")
    }

    fn gather_input(&mut self) {
        let raw_input = read_input_as_vec("./input/day_four.txt");

        let mut found_birth_year        = "".to_string();
        let mut found_issue_year        = "".to_string();
        let mut found_expiration_year   = "".to_string();
        let mut found_height            = "".to_string();
        let mut found_hair_color        = "".to_string();
        let mut found_eye_color         = "".to_string();
        let mut found_passport_id       = "".to_string();
        let mut found_country_id        = "".to_string();

        for (index, line) in raw_input.iter().enumerate() {
            // Split the line by spaces to get key-value pairs as a string
            let mut key_value_pairs: Vec<String> = Vec::new();
            for part in line.split(' ') {
                key_value_pairs.push(part.trim().to_string());
            }

            // Parse passport pairs
            for pair in key_value_pairs {
                let key_value: Vec<&str> = pair.split(':').collect();
                let key = key_value[0];

                // Value may be missing, default to an empty string
                let mut value = "";
                if key_value.len() == 2 {
                    value = key_value[1];
                }

                if key == "byr" {
                    found_birth_year = value.to_string();
                } else if key == "iyr" {
                    found_issue_year = value.to_string();
                } else if key == "eyr" {
                    found_expiration_year = value.to_string();
                } else if key == "hgt" {
                    found_height = value.to_string();
                } else if key == "hcl" {
                    found_hair_color = value.to_string();
                } else if key == "ecl" {
                    found_eye_color = value.to_string();
                } else if key == "pid" {
                    found_passport_id = value.to_string();
                } else if key == "cid" {
                    found_country_id = value.to_string();
                }
            }

            // If the line is an empty line
            if line.trim().is_empty() || index == raw_input.len() - 1 {
                let new_passport = Passport {
                    birth_year:         found_birth_year.to_string(),
                    issue_year:         found_issue_year.to_string(),
                    expiration_year:    found_expiration_year.to_string(),
                    height:             found_height.to_string(),
                    hair_color:         found_hair_color.to_string(),
                    eye_color:          found_eye_color.to_string(),
                    passport_id:        found_passport_id.to_string(),
                    country_id:         found_country_id.to_string()
                };

                // Store a passport no matter if it is valid or not
                if new_passport.is_valid_ignore_country() {
                    self.valid_passports.push(new_passport);
                } else {
                    self.invalid_passports.push(new_passport);
                }

                // Reset tracking variables
                found_birth_year        = "".to_string();
                found_issue_year        = "".to_string();
                found_expiration_year   = "".to_string();
                found_height            = "".to_string();
                found_hair_color        = "".to_string();
                found_eye_color         = "".to_string();
                found_passport_id       = "".to_string();
                found_country_id        = "".to_string();
            }
        }
    }

    // Part one: check how many documents are valid passports
    fn solve_part_one(&self) {
        println!("Answer part one: {} valid passwords (including those without a country ID)", self.valid_passports.len());
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
