use core::time;
use std::{ io, collections::HashMap, thread::sleep };
use itertools::Itertools;

use crate::utils::{get_lines};

fn set_rocks(cave: &mut HashMap<(i32,i32),char>, points: &Vec<(i32,i32)>) {
    for (prev, next) in points.iter().tuple_windows() {
        let mut current = *prev;
        let target = *next;
        while current != target {
            cave.insert(current, '#');
            if target.0 != current.0 {
                current.0 += if target.0 > current.0 {1} else {-1}
            }
            if target.1 != current.1 {
                current.1 += if target.1 > current.1 {1} else {-1}
            }
        }
        cave.insert(current, '#');    
    }
}

fn get_borders(cave: &HashMap<(i32,i32),char>) -> ((i32, i32), (i32, i32)) {
    // Border on x
    let borders_x = cave.keys().map(|x| x.0).sorted().collect_vec();
    let border_x = (*borders_x.first().unwrap(), *borders_x.last().unwrap());
    // Border on x
    let borders_y = cave.keys().map(|x| x.1).sorted().collect_vec();
    let border_y = (*borders_y.first().unwrap(), *borders_y.last().unwrap());
    return (border_x, border_y);
}

// Print cave cleaning screen
fn print_cave(cave: &HashMap<(i32,i32),char>, border_x: (i32, i32), border_y: (i32, i32)) {
    sleep(time::Duration::from_millis(100));
    print!("{esc}c", esc = 27 as char);
    for j in border_y.0-10..border_y.1+2 {
        for i in border_x.0-10..border_x.1+10 {
            let value = cave.get(&(i,j)).unwrap_or(&'.');
            print!("{}", value);
        }
        println!("");
    }
}

#[derive(PartialEq)]
enum GrainStatus {
    Void,
    Rest,
}

fn drop_grain(
    cave: &mut HashMap<(i32,i32),char>, 
    border_x: (i32, i32), 
    border_y: (i32, i32),
    with_floor: bool,
    print: bool,
) -> (GrainStatus, (i32, i32)) {
    let mut init_pos = (500,0);
    let mut prev_init_pos : Option<(i32, i32)> = None;

    loop {
        cave.insert(init_pos, 'o');
        if prev_init_pos != None { 
            cave.remove(&prev_init_pos.unwrap()); 
        }
    
        if print { print_cave(cave, border_x, border_y) }

        let possible = (init_pos.0, init_pos.1 + 1);
        let mut cave_value = *cave.get(&possible).unwrap_or(&'.');

        if with_floor && border_y.1 == possible.1 { cave_value = '#' }

        match cave_value {
            '.' => {
                if possible.1 > border_y.1 { return (GrainStatus::Void, init_pos) }
                prev_init_pos = Some(init_pos);
                init_pos = possible;
                continue;
            },
            _ => {
                let possible = (init_pos.0 - 1, init_pos.1 + 1);
                let mut possible_value = *cave.get(&possible).unwrap_or(&'.');
                if with_floor && border_y.1 == possible.1 { possible_value = '#' }
                if possible_value == '.' {
                    if possible.1 > border_y.1 { return (GrainStatus::Void, init_pos) }
                    prev_init_pos = Some(init_pos);
                    init_pos = possible;
                    continue;
                }

                let possible = (init_pos.0 + 1, init_pos.1 + 1);
                let mut possible_value = *cave.get(&possible).unwrap_or(&'.');
                if with_floor && border_y.1 == possible.1 { possible_value = '#' }
                if possible_value == '.' {
                    if possible.1 > border_y.1 { return (GrainStatus::Void, init_pos) }
                    prev_init_pos = Some(init_pos);
                    init_pos = possible;
                    continue;
                }
                
                return (GrainStatus::Rest, init_pos);
            }
        }
    }
}

fn simulate_drops(
    cave: &mut HashMap<(i32,i32),char>, 
    border_x: (i32, i32), 
    border_y: (i32, i32),
    with_floor: bool,
    print: bool,
) -> i32 {
    let mut count_rest = 0;

    loop {
        let resp = drop_grain(cave, border_x, border_y, with_floor, print);
        if resp.0 == GrainStatus::Void { break; }
        if resp.0 == GrainStatus::Rest { count_rest += 1; }

        if resp.1 == (500,0) { break; }
    }

    return count_rest;
}

pub fn solve() -> Result<(), io::Error> {
    let mut cave : HashMap<(i32,i32),char> = HashMap::new();

    for line in get_lines("day_14") {
        let points : Vec<(i32,i32)> = line.split(" -> ")
            .map(|x| x.split_once(",")
                    .map(
                        |x2| (x2.0.parse::<i32>().unwrap(), x2.1.parse::<i32>().unwrap())
                    ).unwrap()).collect_vec();

        set_rocks(&mut cave, &points);
    }

    let (border_x, border_y) = get_borders(&cave);


    let count_rest = simulate_drops(&mut cave.clone(), border_x, border_y, false, true);
    // let count_rest2 = simulate_drops(&mut cave, border_x, (border_y.0, border_y.1+2), true, false);

    println!("First Star: {:?}", count_rest);
    // println!("Second Star: {:?}", count_rest2);

    return Ok(())
}