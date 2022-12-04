use std::{ io };
use itertools::Itertools;

use crate::utils;

fn full_overlap(pair: ((i32,i32),(i32,i32))) -> i32 {
    let (r1,r2) = pair;
   if r1.0 >= r2.0 && r1.1 <= r2.1 { return 1 }
   if r2.0 >= r1.0 && r2.1 <= r1.1 { return 1 }
    0
}

fn partial_overlap(pair: ((i32,i32),(i32,i32))) -> i32 {
let (r1,r2) = pair;
   if (r1.0..=r1.1).contains(&r2.0) || (r1.0..=r1.1).contains(&r2.1) { return 1 }
   if (r2.0..=r2.1).contains(&r1.0) || (r2.0..=r2.1).contains(&r1.1) { return 1 }
    0
}

fn count_overlaps(pairs: &Vec<((i32,i32),(i32,i32))>, fully: bool) -> i32 {
    match fully {
        true => return pairs.iter().map(|pair| full_overlap(*pair)).sum(),
        false => return pairs.iter().map(|pair| partial_overlap(*pair)).sum(),
    }
}

pub fn solve() -> Result<(), io::Error>{
    let pairs: Vec<((i32,i32),(i32,i32))> = utils::get_lines("day_4").map(|line| {
        let splitted: Vec<String> = line.split(',').map(|x| x.to_string()).collect();
        let r1 : (i32,i32) = splitted[0].split('-').map(|x| x.parse::<i32>().unwrap()).next_tuple().unwrap();
        let r2 : (i32,i32) = splitted[1].split('-').map(|x| x.parse::<i32>().unwrap()).next_tuple().unwrap();
        (r1,r2)
    }).collect_vec();
   
    println!("First Star: {}", count_overlaps(&pairs, true));
    println!("Second Star: {}", count_overlaps(&pairs, false));

    return Ok(())
}