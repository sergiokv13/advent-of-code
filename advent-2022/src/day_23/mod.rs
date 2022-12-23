use std::{ io, collections::{HashMap, HashSet, VecDeque} };

use itertools::Itertools;

use crate::utils::{get_lines};

enum Direction {
    North,
    South,
    East,
    West,
}

fn will_move_to(
    elves: &HashSet<(i128,i128)>, 
    elf: (i128, i128), 
    directions: &VecDeque<Direction>
) -> Option<(i128,i128)> {
    if [
        (elf.0+1, elf.1), (elf.0+1, elf.1+1), (elf.0, elf.1+1),(elf.0-1, elf.1+1),
        (elf.0-1, elf.1), (elf.0-1, elf.1-1), (elf.0, elf.1-1), (elf.0+1, elf.1-1)
    ].iter().all(|pos| !elves.contains(pos)) {
        return None;
    }

    for dir in directions {
        match dir {
            Direction::North => {
                if [
                    (elf.0-1, elf.1), (elf.0-1, elf.1+1), (elf.0-1, elf.1-1)
                ].iter().all(|pos| !elves.contains(pos)) {
                    return Some((elf.0-1, elf.1));
                }
            },
            Direction::South => {
                if [
                    (elf.0+1, elf.1), (elf.0+1, elf.1+1), (elf.0+1, elf.1-1)
                ].iter().all(|pos| !elves.contains(pos)) {
                    return Some((elf.0+1, elf.1));
                }
            }
            Direction::West => {
                if [
                    (elf.0, elf.1-1), (elf.0+1, elf.1-1), (elf.0-1, elf.1-1)
                ].iter().all(|pos| !elves.contains(pos)) {
                    return Some((elf.0, elf.1-1));
                }
            },
            Direction::East => {
                if [
                    (elf.0, elf.1+1), (elf.0+1, elf.1+1), (elf.0-1, elf.1+1)
                ].iter().all(|pos| !elves.contains(pos)) {
                    return Some((elf.0, elf.1+1));
                }
            }
        }
    }
    return None;
}

// will return a boolean. true is finished, false is we need to continue simulating
fn simulate_step(
    elves: &HashSet<(i128,i128)>, 
    directions : &mut VecDeque<Direction>
) -> (HashSet<(i128,i128)>, bool) {
    let possible = elves.clone()
        .iter()
        .map(|&elf| will_move_to(&elves, elf, directions))
        .filter(|&v| v!= None)
        .map(|v| v.unwrap())
        .collect_vec();

    if possible.is_empty() { return (elves.clone(), true) } // we don't need more moves

    let mut possible_count : HashMap<(i128,i128), i128> = HashMap::new();
    for p in possible {
        *possible_count.entry(p).or_insert(0) += 1;
    }

    let mut new_elves : HashSet<(i128,i128)> = HashSet::new();
    for elf in elves.clone() {
        let possible = will_move_to(&elves, elf, directions);
        if possible != None {
            if *possible_count.get(&possible.unwrap()).unwrap() == 1{
                new_elves.insert(possible.unwrap());
            } else { new_elves.insert(elf); }
        } else { new_elves.insert(elf); }
    }
    
    let dir = directions.pop_front().unwrap();
    directions.push_back(dir);

    return (new_elves, false); // we need more moves
}
                                                // (ROW_BOTTOM, ROW_TOP), (COL_BOTTOM,COL_TOP)
fn get_borders(elves: &HashSet<(i128,i128)>) -> ((i128,i128),(i128,i128)) {
    let rb = elves.iter().map(|&elf| elf.0).min().unwrap();
    let rt = elves.iter().map(|&elf| elf.0).max().unwrap();
    let cb = elves.iter().map(|&elf| elf.1).min().unwrap();
    let ct = elves.iter().map(|&elf| elf.1).max().unwrap();

    return ((rb,rt),(cb,ct))
}

fn _print_grid(elves: &HashSet<(i128,i128)>) {
    let (rrange, crange) = get_borders(elves);
    for row in rrange.0-1..rrange.1+2 {
        for col in crange.0-1..crange.1+2 {
            if elves.contains(&(row,col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn simulate_n(n:i128, mut elves: HashSet<(i128,i128)>) -> (HashSet<(i128,i128)>, i128) {
    let mut directions : VecDeque<Direction> = VecDeque::from([Direction::North, Direction::South, Direction::West, Direction::East]);
    let mut counter = 0;
    loop {
        // println!("Step: {:?}", counter);
        // println!();
        // print_grid(&elves);
        // println!();

        if counter == n { break }
        let res = simulate_step(&elves, &mut directions);
        elves = res.0;
        if res.1 { break }
        counter += 1;
    }

    return (elves, counter);
}

fn get_empty_tiles(elves: &HashSet<(i128,i128)>) -> i128 {
    let (rrange, crange) = get_borders(elves);
    let area = (1 + rrange.1 - rrange.0) * (1 + crange.1 - crange.0);

    return area - elves.len() as i128;
}

pub fn solve() -> Result<(), io::Error> {
    let mut elves : HashSet<(i128,i128)> = HashSet::new();

    for (row,line) in get_lines("day_23").enumerate() {
       for (col,el) in line.chars().enumerate() {
            if el == '#' { 
                elves.insert((row as i128,col as i128));
            }
       }
    }

    let after_10 = simulate_n(10, elves.clone()).0;
    println!("First Star: {:?} ", get_empty_tiles(&after_10));
    println!("Second Star: {:?} ", simulate_n(i128::MAX, elves).1 + 1);

    return Ok(())
}