use std::{ io };
use itertools::Itertools;

use crate::utils;

pub fn get_stacks_message(stacks: &mut Vec<Vec<char>>, moves: &Vec<(usize,usize,usize)>, reversed: bool) -> Vec<char> {
    for lmove in moves {
        let last_n = stacks[lmove.1-1].len() - lmove.0;

        let mut to_add : Vec<char>;
        
        if reversed { to_add = stacks[lmove.1-1][last_n..].iter().rev().map(|x| *x).collect(); }
        else { to_add = stacks[lmove.1-1][last_n..].iter().map(|x| *x).collect(); }
        
        stacks[lmove.1-1].drain(last_n..);
        stacks[lmove.2-1].append(&mut to_add);
    }
    return stacks.iter().map(|stack| {
        stack[stack.len()-1]
    }).collect_vec();
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_5");
    let mut stacks : Vec<Vec<char>> = Vec::new();
    let mut moves : Vec<(usize,usize,usize)> = Vec::new();

    let mut reading_stacks = true;
    for line in lines {
        if line.is_empty() { reading_stacks = false; continue; }
        if line.starts_with(" 1") { continue; }

        if reading_stacks {
            let mut stack_idx = 0;
            for (idx, lchar) in line.chars().enumerate() {
                if ((idx as i32) - 1) % 4 == 0 {
                    if stacks.len() <= stack_idx { stacks.push(Vec::new())}
                    if lchar != ' ' { stacks[stack_idx].push(lchar) }
                    stack_idx += 1;
                }
            }
        } else {   
            match line.split_whitespace().collect::<Vec<_>>()[..] {
                ["move", amount, "from", from, "to", to] => {
                    moves.push((
                        amount.parse::<usize>().unwrap(),
                        from.parse::<usize>().unwrap(),
                        to.parse::<usize>().unwrap(),
                    ))
                }
                _ => unreachable!(),
            }
        }
    }
    let mut reversed_stacks : Vec<Vec<char>> = stacks
        .into_iter()
        .map(|s| s.iter().map(|c| *c).rev().collect_vec())
        .collect();
    
    println!("First Star: {:?}", get_stacks_message(&mut reversed_stacks.clone(), &moves, true).iter().join(""));
    println!("Second Star: {:?}", get_stacks_message(&mut reversed_stacks, &moves, false).iter().join(""));

    return Ok(())
}