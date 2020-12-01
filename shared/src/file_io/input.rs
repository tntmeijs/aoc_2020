use std::fs;

// Read a file line-by-line and dump each line as an entry in a vector
pub fn read_input_as_vec(path: &str) -> Vec<String> {
    let content  = fs::read_to_string(path).expect("Unable to read file into a string");
    
    let mut input = Vec::new();
    for line in content.lines() {
        input.push(line.to_string());
    }
    
    input
}
