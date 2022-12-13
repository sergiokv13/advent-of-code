use std::{ io, collections::{HashMap, VecDeque, HashSet} };
use itertools::Itertools;

use crate::utils;

fn find_init_and_end(grid: &mut Vec<Vec<char>>) -> ((usize,usize), (usize,usize)) {
    let mut start : (usize,usize) = (0,0);
    let mut end : (usize, usize) = (0,0);
    for rid in 0..grid.len() {
        for cid in 0..grid[rid].len() {
            if grid[rid][cid] == 'S' { 
                start = (rid,cid);
                grid[rid][cid] = 'a';
            }
            if grid[rid][cid] == 'E' { 
                end = (rid,cid);
                grid[rid][cid] = 'z';
            }
        }
    }
    return (start,end)
}

fn get_neighboors(pos: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize,usize)> {
    let curr = grid[pos.0][pos.1];
    let mut neighboors : Vec<(usize,usize)> = Vec::new();
    
    if pos.0 + 1 < grid.len() {
        let possible = grid[pos.0 + 1][pos.1];
        if (curr as u32) <= 1 + (possible as u32) { neighboors.push((pos.0 + 1,pos.1)) }
    }

    if pos.0 >= 1 {
        let possible = grid[pos.0 - 1][pos.1];
        if (curr as u32) <= 1 + (possible as u32) { neighboors.push((pos.0 - 1, pos.1)) }
    }

    if pos.1 + 1 < grid[0].len() {
        let possible = grid[pos.0][pos.1+1];
        if (curr as u32) <= 1 + (possible as u32) { neighboors.push((pos.0,pos.1+1)) }
    }

    if pos.1 >= 1 {
        let possible = grid[pos.0][pos.1-1];
        if (curr as u32) <= 1 + (possible as u32) { neighboors.push((pos.0, pos.1-1)) }
    }

    return neighboors;
}


fn get_min_steps(init: (usize, usize), end: (usize, usize), grid: &Vec<Vec<char>>) -> i32 {
    let mut queue : VecDeque<(usize, usize)> = VecDeque::from([end]);
    let mut visited : HashSet<(usize,usize)> = HashSet::from([end]);
    let mut count : HashMap<(usize, usize), i32> = HashMap::new(); count.insert(end, 0);
   
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        for neighboor in get_neighboors(node, grid) {
            if !visited.contains(&neighboor) {
                let parent_dist = count.get(&node).unwrap();
                count.insert(neighboor, parent_dist+1);
                visited.insert(neighboor);
                queue.push_back(neighboor);
            }
        }
    }

    return *count.get(&init).unwrap();
}

fn get_min_steps_from_any_point(end: (usize, usize), grid: &Vec<Vec<char>>) -> i32 {
    let mut queue : VecDeque<(usize, usize)> = VecDeque::from([end]);
    let mut visited : HashSet<(usize,usize)> = HashSet::from([end]);
    let mut count : HashMap<(usize, usize), i32> = HashMap::new(); count.insert(end, 0);

    let mut best_start : Option<(usize, usize)> = None;
   
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        for neighboor in get_neighboors(node, grid) {
            if !visited.contains(&neighboor) {
                let parent_dist = count.get(&node).unwrap();
                count.insert(neighboor, parent_dist+1);
                visited.insert(neighboor);
                queue.push_back(neighboor);

                if grid[neighboor.0][neighboor.1] == 'a' && best_start == None {
                    best_start = Some((neighboor.0, neighboor.1));
                }
            }
        }
    }
    return *count.get(&best_start.unwrap()).unwrap();
}

pub fn solve() -> Result<(), io::Error> {
    let mut grid : Vec<Vec<char>> = utils::get_lines("day_12").map(|x| x.chars().collect_vec()).collect_vec();
    
    let (init,end) = find_init_and_end(&mut grid);
    println!("First Star: {:?}", get_min_steps(init, end, &grid));
    println!("Second Star: {:?}", get_min_steps_from_any_point(end, &grid));

    return Ok(())
}