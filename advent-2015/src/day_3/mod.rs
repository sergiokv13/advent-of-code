use std::{ io };
use std::collections::HashMap;

use crate::utils;

fn update_coordinates(mut x: i128, mut y:i128, instruction: char) -> (i128,i128) {
    if instruction == '<' { x -= 1 }
    if instruction == '>' { x += 1 }
    if instruction == 'v' { y -= 1 }
    if instruction == '^' { y += 1 }

    return (x,y);
}

fn count_houses(instructions: &str) -> (i128,i128) {
    let mut curr_x : i128 = 0;
    let mut curr_y : i128 = 0;

    let mut santa_curr_x : i128 = 0;
    let mut santa_curr_y : i128 = 0;

    let mut robo_curr_x : i128 = 0;
    let mut robo_curr_y : i128 = 0;

    let mut santacounter = HashMap::from([
        ((curr_x, curr_y), 1)
    ]);

    let mut bothsantascounter = HashMap::from([
        ((robo_curr_x, robo_curr_y), 2)
    ]);

    for (idx, instruction) in instructions.chars().enumerate() {
        (curr_x, curr_y) = update_coordinates(curr_x, curr_y, instruction);

        if idx % 2 == 0 {
            (santa_curr_x, santa_curr_y) = update_coordinates(santa_curr_x, santa_curr_y, instruction);
            *bothsantascounter.entry((santa_curr_x, santa_curr_y)).or_insert(0) += 1;
        } else {
            (robo_curr_x, robo_curr_y) = update_coordinates(robo_curr_x, robo_curr_y, instruction);
            *bothsantascounter.entry((robo_curr_x, robo_curr_y)).or_insert(0) += 1;
        }

        *santacounter.entry((curr_x, curr_y)).or_insert(0) += 1;
    }

    return (santacounter.len() as i128, bothsantascounter.len() as i128)
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_3");
    for line in lines {
        let (s,r) = count_houses(&line);
        println!("First Star: {}", s);
        println!("Second Star: {}", r);
    }

    return Ok(())
}