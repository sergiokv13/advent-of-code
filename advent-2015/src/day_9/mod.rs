use std::{ io, collections::HashSet };
use crate::utils::{self};
use std::collections::HashMap;
use itertools::Itertools;

fn get_min_dist(paths: &HashMap<(String,String), i128>, cities: &HashSet<String>) -> (i128,i128) {
    let mut min_dist : i128 = i128::MAX;
    let mut max_dist : i128 = i128::MIN;

    for comb in cities.iter().permutations(cities.len()) {
        let mut local_dist = Some(0 as i128);
    
        let mut origin = 0;
        let mut dest = 1;

        loop {
            let path_str = (
                comb[origin].to_string(), 
                comb[dest].to_string()
            );

            if paths.contains_key(&path_str) {
                local_dist = Some(local_dist.unwrap() + paths.get(&path_str).unwrap());
            } else {
                println!("no route between {} {}", path_str.0, path_str.1);
                local_dist = None;
                break;
            }
        
            origin += 1; dest += 1;
            if dest >= comb.len() { break }
        }

        if local_dist != None {
            min_dist = min_dist.min(local_dist.unwrap());
            max_dist = max_dist.max(local_dist.unwrap());
        }
    }

    return (min_dist, max_dist);
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_9");
    let mut paths : HashMap<(String,String), i128> = HashMap::new();
    let mut cities : HashSet<String> = HashSet::new();

    for line in lines {
        let splitted : Vec<&str> = line.split(" = ").collect(); 
        let distance = splitted[1].parse::<i128>().unwrap();
        
        let path_str = splitted[0].to_string();
        let splitted : Vec<&str> = path_str.split(" to ").collect(); 

        cities.insert(splitted[0].to_string());
        cities.insert(splitted[1].to_string());
                
        paths.insert((splitted[0].to_string(), splitted[1].to_string()), distance);
        paths.insert((splitted[1].to_string(), splitted[0].to_string()), distance);
    }

    let sol = get_min_dist(&paths, &cities);

    println!("First Star: {}", sol.0);
    println!("Second Star: {}", sol.1);

    return Ok(())
}