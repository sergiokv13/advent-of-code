use std::{ io, collections::HashSet };

use itertools::Itertools;

use crate::utils;

fn get_char_priority(c: char) -> i32 {
    match c {
        'a'..='z' => return c as i32 - 96,
        _ => return c as i32 - 38,
    }
}

fn get_priority_sum(rucksacks: &Vec<String>) -> i32 {
    return rucksacks.iter().map(|rucksack| {
        let rlen = rucksack.len() / 2;
        let comp1_set : HashSet<char> = rucksack[..rlen].to_string().chars().collect();
        let comp2_set : HashSet<char> = rucksack[rlen..].to_string().chars().collect();
        for repeated in comp1_set.intersection(&comp2_set) {
            return get_char_priority(*repeated);
        }
        0
    }).sum();
}

fn get_batches_priority_sum(rucksacks: &Vec<String>) -> i32 {
    return rucksacks.chunks(3).map(|g| {
        let comp1_set : HashSet<char> = g[0].chars().collect();
        let comp2_set : HashSet<char> = g[1].chars().collect();
        let comp3_set : HashSet<char> = g[2].chars().collect();
        let first_intersection : HashSet<char> = comp1_set.intersection(&comp2_set).map(|c| *c).collect();

        for repeated in first_intersection.intersection(&comp3_set) {
           return get_char_priority(*repeated);
        }
        0
    }).sum();
}

pub fn solve() -> Result<(), io::Error>{
    let rucksacks = utils::get_lines("day_3").collect_vec();
    println!("First Star: {}", get_priority_sum(&rucksacks));
    println!("Second Star: {}", get_batches_priority_sum(&rucksacks));

    return Ok(())
}