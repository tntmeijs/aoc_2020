use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

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

fn string_to_u32(input: &str) -> u32 {
    input.trim().parse().expect("Unable to convert from string to u32")
}

impl Passport {
    fn passes_basic_validation(&self) -> bool {
        !self.birth_year.is_empty()         &&
        !self.issue_year.is_empty()         &&
        !self.expiration_year.is_empty()    &&
        !self.height.is_empty()             &&
        !self.hair_color.is_empty()         &&
        !self.eye_color.is_empty()          &&
        !self.passport_id.is_empty()
    }

    fn has_valid_birth_year(&self) -> bool {
        if self.birth_year.len() != 4 {
            return false;
        }

        let year = string_to_u32(&self.birth_year);
        year >= 1920 && year <= 2002
    }

    fn has_valid_issue_year(&self) -> bool {
        if self.birth_year.len() != 4 {
            return false;
        }

        let year = string_to_u32(&self.issue_year);
        year >= 2010 && year <= 2020
    }

    fn has_valid_expiration_year(&self) -> bool {
        if self.expiration_year.len() != 4 {
            return false;
        }

        let year = string_to_u32(&self.expiration_year);
        year >= 2020 && year <= 2030
    }

    fn has_valid_height(&self) -> bool {
        if self.height.contains("cm") {
            let parts = self.height.split("cm").collect::<Vec<&str>>();
            let height = string_to_u32(parts[0]);
            height >= 150 && height <= 193
        } else if self.height.contains("in") {
            let parts = self.height.split("in").collect::<Vec<&str>>();
            let height = string_to_u32(parts[0]);
            height >= 59 && height <= 76
        } else {
            false
        }
    }

    fn has_valid_hair_color(&self) -> bool {
        if self.hair_color.len() != 7 || self.hair_color.chars().nth(0).unwrap() != '#' {
            return false;
        }

        let hex_code = self.hair_color[1..7].to_string();

        // Ensure the hexadecimal color code is valid
        for character in hex_code.chars() {
            match character {
                '0' => (),
                '1' => (),
                '2' => (),
                '3' => (),
                '4' => (),
                '5' => (),
                '6' => (),
                '7' => (),
                '8' => (),
                '9' => (),
                'a' => (),
                'b' => (),
                'c' => (),
                'd' => (),
                'e' => (),
                'f' => (),
                _ => return false
            }
        }

        true
    }

    fn has_valid_eye_color(&self) -> bool {
        self.eye_color == "amb" ||
        self.eye_color == "blu" ||
        self.eye_color == "brn" ||
        self.eye_color == "gry" ||
        self.eye_color == "grn" ||
        self.eye_color == "hzl" ||
        self.eye_color == "oth"
    }

    fn has_valid_passport_id(&self) -> bool {
        self.passport_id.len() == 9
    }

    fn passes_strict_validation(&self) -> bool {
        // Need to have at least some content in the variables
        if !self.passes_basic_validation() {
            return false;
        }

        let valid_birth_year = self.has_valid_birth_year();
        let valid_issue_year = self.has_valid_issue_year();
        let valid_expiration_year = self.has_valid_expiration_year();
        let valid_height = self.has_valid_height();
        let valid_hair_color = self.has_valid_hair_color();
        let valid_eye_color = self.has_valid_eye_color();
        let valid_passport_id = self.has_valid_passport_id();

        valid_birth_year      &&
        valid_issue_year      &&
        valid_expiration_year &&
        valid_height          &&
        valid_hair_color      &&
        valid_eye_color       &&
        valid_passport_id
    }
}

pub struct Day04 {
    parsed_passports: Vec<Passport>
}

impl Day04 {
    pub fn new() -> Day04 {
        Day04 { parsed_passports: Vec::new() }
    }
}

impl PuzzleTrait for Day04 {
    fn print_info(&self) {
        println!("DAY 4 - PASSPORT PROCESSING")
    }

    fn gather_input(&mut self) {
        let raw_input = read_input_as_vec("./input/day_04.txt");

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
                self.parsed_passports.push(new_passport);

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
        let mut valid_simple_validation = 0;
        for passport in &self.parsed_passports {
            if passport.passes_basic_validation() {
                valid_simple_validation += 1;
            }
        }

        println!("Answer part one: {} valid passwords with basic validation (including those without a country ID)", valid_simple_validation);
    }

    // Part two: check passports against stricter rules
    fn solve_part_two(&self) {
        let mut valid_strict_validation = 0;
        for passport in &self.parsed_passports {
            if passport.passes_strict_validation() {
                valid_strict_validation += 1;
            }
        }

        println!("Answer part two: {} valid passwords with strict validation (including those without a country ID)", valid_strict_validation);
    }
}
