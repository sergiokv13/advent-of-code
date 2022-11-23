use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

pub fn get_lines(day: &str) -> impl Iterator<Item = String> {
    let file = File::open(format!("src/{}/input.txt", day)).expect(
        "Unable to find file"
    );

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let res = lines.map(|l| l.unwrap());
    return res
}

pub fn print_map_str_str(lmap : &HashMap<String,String>) {
    for (key, value) in lmap {
        println!("{}: {}", key, value);
    }
} 