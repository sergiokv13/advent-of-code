use std::{ io };
use itertools::Itertools;
use crate::utils;

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_1");
    let mut calories : Vec<Vec<i32>> = Vec::new();

    let mut curr_vec : Vec<i32> = Vec::new();
    for line in lines {
        match line {
            x if x.is_empty() => {
                calories.push(curr_vec);
                curr_vec = Vec::new();
            }
            _ => curr_vec.push(line.parse::<i32>().unwrap())
        }
    }
    calories.push(curr_vec);

    let sorted_vec : Vec<i32> = calories
        .into_iter()
        .map(|x| x.into_iter().sum::<i32>())
        .sorted().rev().collect();

    let max_calories = sorted_vec[0];
    let max_three : i32 = sorted_vec[0..3].iter().sum();
    
    println!("First Star: {}", max_calories);
    println!("Second Star: {}", max_three);


    return Ok(())
}