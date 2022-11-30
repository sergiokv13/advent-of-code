use std::{ io };
use itertools::Itertools;
use crate::utils::{self};

fn get_possible(
    buckets : &mut Vec<i128>, 
    qty: i128
) -> (i128, i128) {
    let mut count = 0;
    let mut count2 = 0;
    let mut min_lvl = usize::MAX;

    for i in 0..buckets.len() {
        for comb in buckets.iter().combinations(i) {
            if comb.into_iter().map(|x| *x).sum::<i128>() == qty {
                count += 1;
                if count2 == 0 || min_lvl == i {
                    count2 += 1;
                    min_lvl = i;
                }
            }
        }
    }
    
    return (count, count2);
}

pub fn solve() -> Result<(), io::Error> {
    let lines = utils::get_lines("day_17");
    let mut buckets : Vec<i128> = Vec::new();

    for line in lines { 
        let val = line.parse::<i128>().unwrap();
        buckets.push(val);
    }

    let res = get_possible(&mut buckets, 150);
    println!("First Star: {:?}", res.0);
    println!("First Star: {:?}", res.1);
    
    return Ok(())
}