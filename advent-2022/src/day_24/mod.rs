use std::{ io, collections::{HashSet, VecDeque, HashMap} };
// use core::{time};
// use std::{ thread::sleep };

use itertools::Itertools;

use crate::utils::{get_lines};

// fn print_grid(
//     pos: (i128,i128),
//     blizzards : &HashSet<(i128,i128, char)>, 
//     walls_row: (i128, i128), 
//     walls_col: (i128, i128)
// ) {
//     sleep(time::Duration::from_millis(1000));
//     print!("{esc}c", esc = 27 as char);

//     for row in walls_row.0..walls_row.1+1 {
//         for col in walls_col.0..walls_col.1+1 {
//             if (row,col) == pos{ print!("E"); }
//             else if blizzards.contains(&(row,col, '>')) { print!(">") }
//             else if blizzards.contains(&(row,col, '<')) { print!("<") }
//             else if blizzards.contains(&(row,col, 'v')) { print!("v") }
//             else if blizzards.contains(&(row,col, '^')) { print!("^") }
//             else if row == walls_row.0 || row == walls_row.1 { print!("#") }
//             else if col == walls_col.0 || col == walls_col.1 { print!("#") }
//             else { print!(".") }
//         }
//         println!();
//     }
// }

fn get_next_blizzard_pos(
    blizzard : (i128,i128, char),
    walls_row: (i128, i128),
    walls_col: (i128, i128),
) -> (i128,i128, char) {
    let mut _possible = (0,0, '>');

    match blizzard.2 {
        '>' => { _possible = (blizzard.0, blizzard.1 + 1, blizzard.2); },
        '<' => { _possible = (blizzard.0, blizzard.1 - 1, blizzard.2); },
        '^' => { _possible = (blizzard.0 - 1, blizzard.1, blizzard.2); },
        'v' => { _possible = (blizzard.0 + 1, blizzard.1, blizzard.2); },
        _ => unreachable!(),
    }

    // make _possible valid if not the case
                            
    if _possible.0 <= walls_row.0 { _possible.0 = walls_row.1 - 1 }
    if _possible.0 >= walls_row.1 { _possible.0 = walls_row.0 + 1 }
    if _possible.1 <= walls_col.0 { _possible.1 = walls_col.1 - 1 }
    if _possible.1 >= walls_col.1 { _possible.1 = walls_col.0 + 1 }

    return _possible;
}

fn move_blizzards(
    blizzards: &mut HashSet<(i128,i128, char)>,
    walls_row: (i128, i128),
    walls_col: (i128, i128),
) {
    let mut to_insert : Vec<(i128,i128, char)> = Vec::new();

    for blizzard in blizzards.clone() {
        let new_pos = get_next_blizzard_pos(blizzard, walls_row, walls_col);
        blizzards.remove(&blizzard);
        to_insert.push(new_pos);
    }
    for new_pos in to_insert {
        blizzards.insert(new_pos);
    }
}

fn check_valid_pos(
    pos: (i128,i128),
    blizzards: &HashSet<(i128,i128, char)>,
    walls_row: (i128, i128),
    walls_col: (i128, i128),
) -> bool {
    if blizzards.contains(&(pos.0,pos.1,'>')) || 
    blizzards.contains(&(pos.0,pos.1,'<')) ||
    blizzards.contains(&(pos.0,pos.1,'v')) || 
    blizzards.contains(&(pos.0,pos.1,'^')) { 
        return false;
    }

    if pos.0 == walls_row.0 && pos.1 == 1 { return true; }
    if pos.0 == walls_row.1 && pos.1 == walls_col.1 - 1 { return true; }

    if pos.0 <= walls_row.0 || pos.0 >= walls_row.1 { return false; }
    if pos.1 <= walls_col.0 || pos.1 >= walls_col.1 { return false; }

    return true;
}

fn get_neighboors(
    pos: (i128,i128, i128),
    cycle_time: i128,
    time_vs_blizzards: &Vec<HashSet<(i128,i128,char)>>,
    walls_row: (i128, i128),
    walls_col: (i128, i128),
) -> Vec<(i128, i128, i128)> {
    let blizzards = &time_vs_blizzards[pos.2 as usize];
    let new_ts = (pos.2 + 1) % cycle_time;

    let mut possible : Vec::<(i128,i128, i128)> = Vec::new();
    // going back or staying
    if check_valid_pos((pos.0, pos.1 - 1), blizzards, walls_row, walls_col) { possible.push((pos.0, pos.1 - 1, new_ts)); }
    if check_valid_pos((pos.0 - 1, pos.1), blizzards, walls_row, walls_col) { possible.push((pos.0 - 1, pos.1, new_ts)); }
    if check_valid_pos((pos.0, pos.1), &blizzards, walls_row, walls_col) { possible.push((pos.0, pos.1, new_ts)); }
    // move to target
    if check_valid_pos((pos.0, pos.1 + 1), blizzards, walls_row, walls_col) { possible.push((pos.0, pos.1 + 1, new_ts)); }
    if check_valid_pos((pos.0 + 1, pos.1), blizzards, walls_row, walls_col) { possible.push((pos.0 + 1, pos.1, new_ts)); }
    
    return possible;
}

fn get_fewest(
    pos: (i128,i128, i128),
    final_pos: (i128, i128),
    cycle_time: i128,
    time_vs_blizzards: &Vec<HashSet<(i128,i128,char)>>,
    walls_row: (i128, i128),
    walls_col: (i128, i128),
) -> (i128, (i128,i128, i128)) {
    let mut dist : HashMap<(i128,i128,i128), i128> = HashMap::new();
    let mut visited : HashSet<(i128,i128,i128)> = HashSet::new();
    let mut queue : VecDeque<(i128,i128,i128)> = VecDeque::new();
    let mut parent: HashMap<(i128,i128,i128),(i128,i128,i128)> = HashMap::new();
    
    queue.push_back(pos);
    visited.insert(pos);
    dist.insert(pos, 0);
    parent.insert(pos, pos);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let current_dist = *dist.get(&current).unwrap();

        if current.0 == final_pos.0 && current.1 == final_pos.1 { 
            let mut curr_parent = &current;
            while *curr_parent != pos {
                // println!("-> {:?}", curr_parent);
                curr_parent = parent.get(&curr_parent).unwrap();
            }
            // println!("-> {:?}", curr_parent);
            return (current_dist, current); 
        }

        for neighboor in get_neighboors(current, cycle_time, time_vs_blizzards, walls_row, walls_col) {
            if !visited.contains(&neighboor) {
                queue.push_back(neighboor);
                visited.insert(neighboor);
                dist.insert(neighboor, current_dist + 1);

                parent.insert(neighboor, current);
            }
        }
    }

    return (0, pos);
}

fn precompute_times(
    blizzards: &HashSet<(i128,i128, char)>,
    walls_row: (i128, i128),
    walls_col: (i128, i128),
) -> (Vec<HashSet<(i128,i128, char)>>, i128) {
    let mut time_vs_blizzards : Vec<HashSet<(i128,i128,char)>> = Vec::new();

    let key = format!("{:?}", blizzards.iter().sorted());
    let mut cycle_time = 0;
    let mut new_blizzards = blizzards.clone();

    loop {
        time_vs_blizzards.push(new_blizzards.clone());
        move_blizzards(&mut new_blizzards, walls_row, walls_col);
        let new_key = format!("{:?}", new_blizzards.iter().sorted());
        if key == new_key { break }
        cycle_time += 1;
    }
    return (time_vs_blizzards, cycle_time);
}

pub fn solve() -> Result<(), io::Error> {
    let mut blizzards : HashSet<(i128,i128, char)> = HashSet::new();

    for (row,line) in get_lines("day_24").enumerate() {
       for (col,el) in line.chars().enumerate() {
            if el == '>' || el == '<' || el == 'v' || el == '^'  { 
                blizzards.insert((row as i128,col as i128, el)); 
            }
       }
    }

    let rl = get_lines("day_24").collect_vec().len()-1;
    let cl = get_lines("day_24").collect_vec()[0].len()-1;
    let walls_row : (i128, i128) = (0, rl as i128);
    let walls_col : (i128, i128) = (0, cl as i128);

    let (time_vs_blizzards, cycle_time) = precompute_times(&blizzards, walls_row, walls_col);

    let (first, first_pos) = get_fewest((0,1,1), (walls_row.1,walls_col.1 - 1), cycle_time, &time_vs_blizzards,  walls_row,  walls_col);
    println!("Second Star: {:?} ", first);
    let (second, second_pos) = get_fewest(first_pos, (0,1), cycle_time, &time_vs_blizzards,  walls_row,  walls_col);
    // going back!
    let (third, _) = get_fewest(second_pos, (walls_row.1,walls_col.1 - 1), cycle_time, &time_vs_blizzards,  walls_row,  walls_col);
    // For each one let's remove the initial pos and the final pos
    println!("Second Star: {:?} ", first + second + third - (3 * 2));

    return Ok(())
}