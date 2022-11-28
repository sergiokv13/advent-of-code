use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, fmt::Debug};

pub fn get_lines(day: &str) -> impl Iterator<Item = String> {
    let file = File::open(format!("src/{}/input.txt", day)).expect(
        "Unable to find file"
    );

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let res = lines.map(|l| l.unwrap());
    return res
}

pub fn print_map<T:Debug,V:Debug>(lmap : &HashMap<T,V>) {
    for (key, value) in lmap {
        println!("{:?}: {:?}", key, value);
    }
} 