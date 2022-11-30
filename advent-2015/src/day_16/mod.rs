use std::{ io, collections::HashMap };
use serde_json;

use crate::utils::{self};

fn get_aunt(
    aunts: &mut Vec<HashMap<String, i32>>, 
    scan: HashMap<String, i32>,
    with_comparison : bool,
) -> i32 {
    for attr in scan.keys() {
        match attr {
            x if (x == "cats" || x == "trees" && with_comparison) => aunts.retain(|x| x.get(attr).unwrap_or(scan.get(attr).unwrap()) >= scan.get(attr).unwrap()),
            x if (x == "pomeranians" || x == "goldfish" && with_comparison) => aunts.retain(|x| x.get(attr).unwrap_or(scan.get(attr).unwrap()) <= scan.get(attr).unwrap()),
            _ => aunts.retain(|x| x.get(attr).unwrap_or(scan.get(attr).unwrap()) == scan.get(attr).unwrap()),
        }
    }

    return aunts[0].get("id").unwrap() + 1;
}

pub fn solve() -> Result<(), io::Error> {
    let lines = utils::get_lines("day_16");
    let mut aunts : Vec<HashMap<String, i32>> = Vec::new();

    let scan : HashMap<String, i32> = HashMap::from([
        ("children",3),
        ("cats",7),
        ("samoyeds",2),
        ("pomeranians",3),
        ("akitas",0),
        ("vizslas",0),
        ("goldfish",5),
        ("trees",3),
        ("cars",2),
        ("perfumes",1)].map(|x| (x.0.to_string(), x.1)));

    for (idx, line) in lines.enumerate() { 
        let colon_idx = line.find(":").unwrap();
        let raw_json = format!("{{\"{}, \"id\": {}}}", line[colon_idx+2..].replace(":", "\":").replace(", ", ", \""), idx);
        let json: HashMap<String, i32> = serde_json::from_str(&raw_json)?;
        aunts.push(json);
    }
    
    println!("First Star: {:?}", get_aunt(&mut aunts.clone(), scan.clone(), false));
    println!("Second Star: {:?}", get_aunt(&mut aunts.clone(), scan, true));
    
    return Ok(())
}