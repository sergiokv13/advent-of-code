use std::{ io };

use crate::utils;

fn get_next(val : Vec<char>) -> Vec<char> {
    let mut curr_char = val[0] as char;
    let mut curr_idx = 1;
    let mut count = 1;

    let mut new_val : Vec<char> = Vec::new();
    loop {
        if curr_idx >= val.len() {
            break;
        }

        match val[curr_idx] as char {
            x if x == curr_char => { count += 1 }
            _ => {
                let mut count_str : Vec<char> = format!("{}", count).chars().collect();
                new_val.append(&mut count_str);
                new_val.push(curr_char);
                curr_char = val[curr_idx] as char; 
                count = 1
            }
        }
        curr_idx += 1;
    }
    let mut count_str : Vec<char> = format!("{}", count).chars().collect();
    new_val.append(&mut count_str);
    new_val.push(curr_char);

    return new_val;
}

pub fn solve() -> Result<(), io::Error> {
    let lines = utils::get_lines("day_10");
    let mut init_val : Vec<char> = Vec::new();
    
    let mut _val : Vec<char> = Vec::new();

    for line in lines { 
        let vec_line : Vec<char> = line.chars().collect();
        init_val = vec_line; 
    }
    
    _val = init_val.clone();
    for _idx in 0..40 { _val = get_next(_val) }
    println!("First Star: {}", _val.len());

    _val = init_val.clone();
    
    for _idx in 0..50 { _val = get_next(_val) }
    println!("Second Star: {}", _val.len());

    return Ok(())
}