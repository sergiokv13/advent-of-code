use std::{ io };
use std::str;
use crate::utils;

const GRID_SIZE : usize = 1000;

fn perform_action(init: Vec<usize>, end: Vec<usize>, command: &str, grid: &mut [[i32; GRID_SIZE]; GRID_SIZE]) {
    for idx in init[0]..end[0]+1 {
        for idx2 in init[1]..end[1]+1 {
            match command {
                "on" => grid[idx][idx2] = 1,
                "off" => grid[idx][idx2] = 0,
                _ => grid[idx][idx2] = if grid[idx][idx2] == 1 {0} else {1},
            }
        }
    }
}

fn perform_action2(init: Vec<usize>, end: Vec<usize>, command: &str, grid: &mut [[i32; GRID_SIZE]; GRID_SIZE]) {
    for idx in init[0]..end[0]+1 {
        for idx2 in init[1]..end[1]+1 {
            match command {
                "on" => grid[idx][idx2] += 1,
                "off" => grid[idx][idx2] = if grid[idx][idx2] - 1 <= 0 { 0 } else { grid[idx][idx2] - 1 },
                _ => grid[idx][idx2] += 2,
            }
        }
    }
}

fn count_on(grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> i32 {
    let mut counter = 0;
    for idx in 0..GRID_SIZE {
        for idx2 in 0..GRID_SIZE {
            counter += grid[idx][idx2]
        }
    }
    return counter;
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_6");
    let mut grid = [[0 ; GRID_SIZE] ; GRID_SIZE];
    let mut grid2 = [[0 ; GRID_SIZE] ; GRID_SIZE];

    for line in lines {
        let command: &str;
        let coor: Vec<&str>;
        let rpl_str: String;
    
        if line.contains("turn on") {
            rpl_str = line.replace("turn on ", "");
            command = "on";
        }else if line.contains("turn off") {
            rpl_str = line.replace("turn off ", "");
            command = "off";
        } else {
            rpl_str = line.replace("toggle ", "");
            command = "toggle";
        }
        let splitted = rpl_str.split(" through ");
        coor  = splitted.collect();

        let init : Vec<usize> = coor[0].split(",").map(|n| n.parse::<usize>().unwrap()).collect();
        let end : Vec<usize> = coor[1].split(",").map(|n| n.parse::<usize>().unwrap()).collect();

        perform_action(init.clone(), end.clone(), command, &mut grid);
        perform_action2(init, end, command, &mut grid2);
    }

    println!("First Star: {}", count_on(&grid));
    println!("Second Star: {}", count_on(&grid2));

    return Ok(())
}