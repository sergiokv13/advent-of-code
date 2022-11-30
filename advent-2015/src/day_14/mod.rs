use std::{ io, collections::{HashMap} };
use crate::utils::{self};


// vel, time_flying, time_resting
fn get_pos(specs : (i32, i32, i32), time : i32) -> i32 {
    let mut curr_time = time;
    let mut curr_pos = 0;
    loop {
        if curr_time <= 0 { break }
        let time_flying = specs.1.min(curr_time);
        curr_time -= time_flying;
        curr_pos += time_flying * specs.0;
        if curr_time <= 0 { break }
        curr_time -= specs.2;
    }
    return curr_pos;
}

fn get_fastest(specs: &HashMap<String, (i32, i32, i32)>, time: i32) -> Vec<(i32,String)> {
    let mut traveled : HashMap<String, i32> = HashMap::new();

    for key in specs.keys() {
        let el = specs.get(key).unwrap();
        let deer_traveled = get_pos(*el, time);
        traveled.insert(key.to_string(), deer_traveled); 
    }

    let fastest = *traveled.values().into_iter().max().unwrap();

    let mut winners : Vec<(i32,String)> = Vec::new();
    for key in specs.keys() {
        if traveled[key] == fastest {
            winners.push((fastest, key.to_string()))
        }
    }

    return winners;
}

fn get_fastest_with_award(specs: &HashMap<String, (i32, i32, i32)>, time: i32) -> i32 {
    let mut awards : HashMap<String, i32> = HashMap::new();
    for i in 1..time+1 {
        for (_score, name) in get_fastest(&specs, i) {
            *awards.entry(name).or_insert(0) += 1;
        }
    }
    
    return *awards.values().into_iter().max().unwrap();
}

pub fn solve() -> Result<(), io::Error> {
    let lines = utils::get_lines("day_14");
    let mut specs : HashMap<String, (i32, i32, i32)> = HashMap::new(); // vel, time_flying, time_resting

    for line in lines { 
        let clean_line = line
            .replace(" can fly ", " ")
            .replace(" km/s for ", " ")
            .replace(" seconds, but then must rest for ", " ")
            .replace(" seconds.", "");

        let v_line : Vec<&str> = clean_line.split(" ").collect();
        specs.insert(v_line[0].to_string(), (
            v_line[1].parse::<i32>().unwrap(),
            v_line[2].parse::<i32>().unwrap() as i32,
            v_line[3].parse::<i32>().unwrap() as i32
        ));
    }

    println!("First Star: {:?}", get_fastest(&specs, 2503));
    println!("Second Star: {:?}", get_fastest_with_award(&specs, 2503));
    
    return Ok(())
}