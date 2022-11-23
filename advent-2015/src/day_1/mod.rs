use std::{ io };

use crate::utils;

fn get_floor(instructions: &str) -> (i128, Option<i128>) {
    let mut count : i128 = 0;
    let mut first_char = None;

    for (idx, instruction) in instructions.chars().enumerate() {
        if instruction == '(' { count += 1; } 
        else { count -= 1; }

        if count == -1 && first_char.is_none() {
            first_char = Some(idx as i128);
        }
    }
    
    return (count, first_char);
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_1");

    for line in lines {
        let res = get_floor(&line);
        println!("First Star: {}", res.0);
        println!("Second Star: {}", res.1.unwrap() + 1);
    }

    return Ok(())
}