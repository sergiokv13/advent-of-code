use std::{ io, collections::{HashMap,HashSet} };

use itertools::Itertools;

use crate::utils::{get_lines};

fn get_most_pressure(
    curr_valve: String,
    cave: &HashMap<String, Vec<String>>, 
    valve_flow: &HashMap<String, i128>,
    remaining_minutes: i32,
    opened: &mut HashSet<String>,
    memo: &mut HashMap<(String,i32,String),i128>,
    worthy_valves: usize,
) -> i128 {
    let current_flow = opened.iter().map(|key| *valve_flow.get(key).unwrap()).sum::<i128>();
    let opened_string = opened.iter().sorted().join("-");

    let memo_key = (curr_valve.clone(), remaining_minutes, opened_string);
    if memo.contains_key(&memo_key) {
        return *memo.get(&memo_key).unwrap();
    }
    
    if remaining_minutes == 0 { return  0 }
    if remaining_minutes == 1 { return current_flow }
    if opened.len() == worthy_valves { return current_flow * remaining_minutes as i128 }

    let mut best_flow : i128 = i128::MIN;

    // if we leave the valve without opening
    for neighboor in cave.get(&curr_valve).unwrap_or(&Vec::new()) {
        best_flow = best_flow.max(current_flow + get_most_pressure(neighboor.to_string(), cave,valve_flow, remaining_minutes - 1,opened,memo,worthy_valves));
    }

    // If we open the valve (only if val flow is greater than 0)
    // and if it's not already opened
    if !opened.contains(&curr_valve) && *valve_flow.get(&curr_valve).unwrap() > 0 {
        opened.insert(curr_valve.clone());
        for neighboor in cave.get(&curr_valve).unwrap_or(&Vec::new()) {
            best_flow = best_flow.max(
                current_flow * 2 + *valve_flow.get(&curr_valve).unwrap() + get_most_pressure( neighboor.to_string(),  cave,  valve_flow,  remaining_minutes - 2, opened, memo, worthy_valves)
            )
        }
        opened.remove(&curr_valve);
    }
    memo.insert(memo_key, best_flow);

    return best_flow;
}

fn get_most_pressure2(
    curr_valve: [String;2],
    cave: &HashMap<String, Vec<String>>, 
    valve_flow: &HashMap<String, i128>,
    remaining_minutes: i32,
    opened: &mut HashSet<String>,
    memo: &mut HashMap<(String,i32,String),i128>,
    worthy_valves: usize,
) -> i128 {
    if remaining_minutes == 0 { return  0 }
    // We cut this branch, as there is no way less valves will lead us to the answer
    if (worthy_valves - opened.len()) as i32 > remaining_minutes + 1 { return  0 } 

    let current_flow = opened.iter().map(|key| *valve_flow.get(key).unwrap()).sum::<i128>();
    let opened_string = opened.iter().sorted().join("-");
    let curr_valve_string = curr_valve.iter().sorted().join("-");

    let memo_key = (curr_valve_string, remaining_minutes, opened_string);
    if memo.contains_key(&memo_key) {
        return *memo.get(&memo_key).unwrap();
    }

    if remaining_minutes == 1 { return current_flow }
    if opened.len() == worthy_valves { return current_flow * remaining_minutes as i128 }

    let mut best_flow : i128 = i128::MIN;

    let will_open_v1 = !opened.contains(&curr_valve[0]) && *valve_flow.get(&curr_valve[0]).unwrap() > 0;
    let will_open_v2 = !opened.contains(&curr_valve[1]) && *valve_flow.get(&curr_valve[1]).unwrap() > 0;

    for opt in 1..5 {
        match opt {
            // We leave both valves closed and move to our neighboors
            1 => { 
                for neighboor in cave.get(&curr_valve[0]).unwrap_or(&Vec::new()) {
                    for neighboor2 in cave.get(&curr_valve[1]).unwrap_or(&Vec::new()) {
                        best_flow = best_flow.max(
                            current_flow + get_most_pressure2(
                                [neighboor.to_string(), neighboor2.to_string()],  
                                cave,  
                                valve_flow,  
                                remaining_minutes - 1, 
                                opened, 
                                memo, 
                                worthy_valves
                            )
                        )
                    }
                }
            },
            // We open both valves if possible
            2 => { 
                if will_open_v1 && will_open_v2 {
                    opened.insert(curr_valve[0].clone());
                    opened.insert(curr_valve[1].clone());
                    for neighboor in cave.get(&curr_valve[0]).unwrap_or(&Vec::new()) {
                        for neighboor2 in cave.get(&curr_valve[1]).unwrap_or(&Vec::new()) {
                            best_flow = best_flow.max(
                                current_flow * 2 + 
                                *valve_flow.get(&curr_valve[0]).unwrap() + 
                                *valve_flow.get(&curr_valve[1]).unwrap() + 
                                get_most_pressure2(
                                    [neighboor.to_string(), neighboor2.to_string()],  
                                    cave,  
                                    valve_flow,  
                                    remaining_minutes - 2, 
                                    opened, 
                                    memo, 
                                    worthy_valves
                                )
                            );
                        }
                    }
                    opened.remove(&curr_valve[0]);
                    opened.remove(&curr_valve[1]);
                    
                }
            },
            // We try to open first valve and do not open second
            3 => { 
                if will_open_v1 {
                    opened.insert(curr_valve[0].clone());
                    for neighboor2 in cave.get(&curr_valve[1]).unwrap_or(&Vec::new()) {
                        best_flow = best_flow.max(
                            current_flow + 
                            get_most_pressure2(
                                [curr_valve[0].clone(), neighboor2.to_string()],  
                                cave,  
                                valve_flow,  
                                remaining_minutes - 1, 
                                opened, 
                                memo, 
                                worthy_valves
                            )
                        );
                    }
                    opened.remove(&curr_valve[0]);
                }
            },
            // We try to open second first valve and do not open first
            _ => { 
                if will_open_v2 {
                    opened.insert(curr_valve[1].clone());
                    for neighboor in cave.get(&curr_valve[0]).unwrap_or(&Vec::new()) {
                        best_flow = best_flow.max(
                            current_flow + 
                            get_most_pressure2(
                                [neighboor.to_string(), curr_valve[1].clone()],  
                                cave,  
                                valve_flow,  
                                remaining_minutes - 1, 
                                opened, 
                                memo, 
                                worthy_valves
                            )
                        );
                    }
                    opened.remove(&curr_valve[1]);
                }
            }
        }
    }

    memo.insert(memo_key, best_flow);
    return best_flow;
}

pub fn solve() -> Result<(), io::Error> {
    let mut cave : HashMap<String, Vec<String>> = HashMap::new();
    let mut valve_flow : HashMap<String, i128> = HashMap::new();

    for line in get_lines("day_16") {
        let splitted : Vec<String> = line
            .replace("Valve ", "")
            .replace(" has flow rate=", " ")
            .replace("; tunnels lead to valves ", " ")
            .replace("; tunnel leads to valve ", " ")
            .replace(",", "")
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        valve_flow.insert(splitted[0].clone(), splitted[1].parse::<i128>().unwrap());
        
        let mut neighboors : Vec<String> = Vec::new();
        for i in 2..splitted.len() {
            neighboors.push(splitted[i].clone());
        }
        cave.insert(splitted[0].clone(), neighboors);
    }  

    let mut opened = HashSet::new();
    let mut memo: HashMap<(String,i32, String),i128> = HashMap::new();
    println!("First Star: {:?}", get_most_pressure(
        "AA".to_string(), 
        &cave, 
        &valve_flow, 
        30, 
        &mut opened,
        &mut memo,
        valve_flow.values().filter(|x| **x>0).count()
    ));

    let mut opened = HashSet::new();
    let mut memo: HashMap<(String,i32, String),i128> = HashMap::new();
    println!("Second Star: {:?}", get_most_pressure2(
        ["AA".to_string(), "AA".to_string()], 
        &cave, 
        &valve_flow, 
        26, 
        &mut opened,
        &mut memo,
        valve_flow.values().filter(|x| **x>0).count()
    ));

    return Ok(())
}