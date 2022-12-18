use core::{time};
use std::{ io, thread::sleep, borrow::BorrowMut };

use crate::utils::{get_lines};

// This parameter required some fine tuning
const MAX_NUMBER_BEFORE_PATTERN: usize = 10000;

#[derive(Debug, Clone, Copy, PartialEq)]
enum RockType {
    HorizontalLine,
    Plus,
    InverseL,
    VerticalLine,
    Square,
}

fn print_grid(grid : &Vec<[char;7]>) {
    sleep(time::Duration::from_millis(500));
    print!("{esc}c", esc = 27 as char);
    for i in (0..grid.len()).rev() {
        println!("{:?}", grid[i].map(|x| x.to_string()).join(""));
    }
}

fn insert_row( grid : &mut Vec<[char;7]>, row : [char;7], lvl : usize ) {
    if lvl < grid.len() { grid[lvl] = row; return; }
    for i in grid.len()..lvl+1 {
        if lvl == i { grid.push(row)}
        else { grid.push(['.';7])}
    }
}

fn go_down( rock: RockType, grid : &mut Vec<[char;7]>, l: usize, b: usize) -> (usize, bool) {
    if b as i32 - 1 < 0 { return (b, false) }
    match rock {
        RockType::HorizontalLine => {
            // Validate
            for i in l..l+4 {
                if grid[b-1][i] == '#' { return (b, false) }
            }
            // Update grid
            for i in l..l+4 { grid[b][i] = '.'; grid[b-1][i] = '#'; }
        }
        RockType::Plus => {
            // Validate
            if grid[b-1][l+1] == '#' { return (b, false) }
            if grid[b][l] == '#' { return (b, false) }
            if grid[b][l+2] == '#' { return (b, false) }
            // Update grid
            grid[b+1][l] = '.'; grid[b][l] = '#';
            grid[b+1][l+2] = '.'; grid[b][l+2] = '#';
            grid[b][l+1] = '.'; grid[b-1][l+1] = '#';
            grid[b+1][l+1] = '.'; grid[b][l+1] = '#';
            grid[b+2][l+1] = '.'; grid[b+1][l+1] = '#'
        }
        RockType::InverseL => { 
            // Validate
            for i in l..l+3 {
                if grid[b-1][i] == '#' { return (b, false) }
            }
            // Update Grid
            for i in l..l+3 { grid[b][i] = '.'; grid[b-1][i] = '#' }
            grid[b+1][l+2] = '.'; grid[b][l+2] = '#';
            grid[b+2][l+2] = '.'; grid[b+1][l+2] = '#';
        }
        RockType::VerticalLine => { 
            // Validate
            if grid[b-1][l] == '#' { return (b, false) }
            // Update Grid
            grid[b+3][l] = '.'; grid[b-1][l] = '#';
        }
        RockType::Square => { 
            if grid[b-1][l] == '#' { return (b, false) }
            if grid[b-1][l+1] == '#' { return (b, false) }
            // Update Grid
            grid[b+1][l] = '.'; grid[b-1][l] = '#';
            grid[b+1][l+1] = '.'; grid[b-1][l+1] = '#';
        }
    }

    return (b - 1, true);
}

fn go_sideways( rock: RockType, grid : &mut Vec<[char;7]>, cmd: char, l: usize, b: usize) -> (usize, bool) {
    if cmd == '<' && l as i32 - 1 < 0 { return (l, false) }
    match rock {
        RockType::HorizontalLine => {
            // Validate
            if cmd == '>' && l + 4 >= 7 { return (l, false) }

            if cmd == '<' && grid[b][l-1] == '#' { return (l, false) }
            if cmd == '>' && grid[b][l+4] == '#' { return (l, false) }
            // Update grid
            if cmd == '<' { grid[b][l+3] = '.'; grid[b][l-1] = '#' }
            if cmd == '>' { grid[b][l] = '.'; grid[b][l+4] = '#' }
        }
        RockType::Plus => {
            // Validate
            if cmd == '>' && l + 3 >= 7 { return (l, false) }

            if cmd == '<' && grid[b][l] == '#' { return (l, false) }
            if cmd == '<' && grid[b+1][l-1] == '#' { return (l, false) }
            if cmd == '<' && grid[b+2][l] == '#' { return (l, false) }

            if cmd == '>' && grid[b][l+2] == '#' { return (l, false) }
            if cmd == '>' && grid[b+1][l+3] == '#' { return (l, false) }
            if cmd == '>' && grid[b+2][l+2] == '#' { return (l, false) }

            // Update grid
            grid[b][l+1] = '.'; grid[b+1][l] = '.'; grid[b+1][l+1] = '.';
            grid[b+1][l+2] = '.'; grid[b+2][l+1] = '.';
            if cmd == '>' {
                grid[b][l+2] = '#'; grid[b+1][l+1] = '#'; grid[b+1][l+2] = '#';
                grid[b+1][l+3] = '#'; grid[b+2][l+2] = '#';
            } else {
                grid[b][l] = '#'; grid[b+1][l-1] = '#'; grid[b+1][l] = '#';
                grid[b+1][l+1] = '#'; grid[b+2][l] = '#';
            }
        }
        RockType::InverseL => { 
            // Validate
            if cmd == '>' && l + 3 >= 7 { return (l, false) }

            if cmd == '<' && grid[b][l-1] == '#' { return (l, false) };
            if cmd == '<' && grid[b+1][l+1] == '#' { return (l, false) };
            if cmd == '<' && grid[b+2][l+1] == '#' { return (l, false) };

            if cmd == '>' && grid[b][l+3] == '#' { return (l, false) };
            if cmd == '>' && grid[b+1][l+3] == '#' { return (l, false) };
            if cmd == '>' && grid[b+2][l+3] == '#' { return (l, false) };
        
            // Update Grid
            grid[b][l] = '.'; grid[b][l+1] = '.'; grid[b][l+2] = '.';
            grid[b+1][l+2] = '.'; grid[b+2][l+2] = '.';
            if cmd == '>' {
                grid[b][l+1] = '#'; grid[b][l+2] = '#'; grid[b][l+3] = '#';
                grid[b+1][l+3] = '#'; grid[b+2][l+3] = '#';
            } else {
                grid[b][l-1] = '#'; grid[b][l] = '#'; grid[b][l+1] = '#';
                grid[b+1][l+1] = '#'; grid[b+2][l+1] = '#';
            }
        }
        RockType::VerticalLine => { 
            // Validate
            if cmd == '>' && l + 1 >= 7 { return (l, false) }

            for i in b..b+4 {
                if cmd == '<' && grid[i][l-1] == '#' { return (l, false) }
                if cmd == '>' && grid[i][l+1] == '#' { return (l, false) }
            }
            // Update Grid
            for i in b..b+4 {
                if cmd == '<' { grid[i][l-1] = '#' }
                if cmd == '>' { grid[i][l+1] = '#' }
                grid[i][l] = '.';
            }
        }
        RockType::Square => { 
            if cmd == '>' && l + 2 >= 7 { return (l, false) }

            if cmd == '<' && grid[b][l-1] == '#' { return (l, false) }
            if cmd == '<' && grid[b+1][l-1] == '#' { return (l, false) }
            if cmd == '>' && grid[b][l+2] == '#' { return (l, false) }
            if cmd == '>' && grid[b+1][l+2] == '#' { return (l, false) }
            // Update Grid
            if cmd == '<' {
                grid[b][l-1] = '#'; grid[b][l+1] = '.';
                grid[b+1][l-1] = '#'; grid[b+1][l+1] = '.';
            } else {
                grid[b][l+2] = '#'; grid[b][l] = '.';
                grid[b+1][l+2] = '#'; grid[b+1][l] = '.';            
            }
        }
    }

    if cmd == '<' { return (l - 1, true); }
    return (l + 1, true);
}

fn simulate_rock_fall(
    rock : RockType,  
    grid : &mut Vec<[char;7]>,  
    last_lvl : usize, 
    mut commands: impl Iterator<Item = char>,
    print : bool,
) -> usize {
    let mut left_edge = 2 as usize;
    let mut bottom_edge = last_lvl + 3;

    // Rock falls 3 steps above last lvl
    match rock {
        RockType::HorizontalLine => { 
            insert_row(grid, ['.','.','#','#','#','#','.'], bottom_edge) 
        }
        RockType::Plus => { 
            insert_row(grid, ['.','.','.','#','.','.','.'], bottom_edge+2);
            insert_row(grid, ['.','.','#','#','#','.','.'], bottom_edge+1);
            insert_row(grid, ['.','.','.','#','.','.','.'], bottom_edge); 
        }
        RockType::InverseL => { 
            insert_row(grid, ['.','.','.','.','#','.','.'], bottom_edge+2);
            insert_row(grid, ['.','.','.','.','#','.','.'], bottom_edge+1);
            insert_row(grid, ['.','.','#','#','#','.','.'], bottom_edge); 
        }
        RockType::VerticalLine => { 
            insert_row(grid, ['.','.','#','.','.','.','.'], bottom_edge+3);
            insert_row(grid, ['.','.','#','.','.','.','.'], bottom_edge+2);
            insert_row(grid, ['.','.','#','.','.','.','.'], bottom_edge+1); 
            insert_row(grid, ['.','.','#','.','.','.','.'], bottom_edge); 
        }
        RockType::Square => { 
            insert_row(grid, ['.','.','#','#','.','.','.'], bottom_edge+1);
            insert_row(grid, ['.','.','#','#','.','.','.'], bottom_edge);
        }
    }

    let mut moved = true;
    while moved {
        if print { print_grid(grid); }
        (left_edge, _) = go_sideways(rock, grid, commands.next().unwrap(), left_edge, bottom_edge);
        if print { print_grid(grid); }
        (bottom_edge, moved) = go_down(rock.clone(), grid, left_edge, bottom_edge);
    }

    for i in (0..grid.len()).rev() {
        if grid[i].contains(&'#') { return i + 1 }
    }
    return 0;
}

fn simulate_n(
    mut commands: impl Iterator<Item = char>,
    number_of_rocks: i128,
) -> i128 {
    let mut rocks = [
        RockType::HorizontalLine, RockType::Plus, RockType::InverseL, 
        RockType::VerticalLine, RockType::Square
    ].iter().cycle();
    let mut grid: Vec<[char;7]> = Vec::new();
    let mut last_lvl = 0;
    let mut lvl_by_rock : Vec<usize> = Vec::new();
    let mut counter: i128 = 0;

    let mut lvl_diff : Vec<usize> = Vec::new();

    let mut offset : usize = 0;
    let mut pattern_len : usize = 0;

    let mut calculate_with_pattern = true;

    loop {
        let curr_rock = *rocks.next().unwrap();

        last_lvl = simulate_rock_fall(
            curr_rock, 
            &mut grid, 
            last_lvl, 
            commands.borrow_mut(),
            false,
        );

        if lvl_by_rock.is_empty() { lvl_diff.push(last_lvl)}
        else { lvl_diff.push(last_lvl - lvl_by_rock[counter as usize - 1]) }
        lvl_by_rock.push(last_lvl);
        counter += 1;

        if counter == number_of_rocks { 
            calculate_with_pattern = false;
            break;
        }

        if counter == MAX_NUMBER_BEFORE_PATTERN as i128 { break }
    }

    if calculate_with_pattern {

        // we need to get the offset and the pattern len
        // we will do this by using windows and finding a contigous seq
        for o in 0..lvl_diff.len() {

            for i in 2..lvl_diff.len() {
                let mut chunks = lvl_diff[o..].chunks_exact(i);
                if chunks.len() < 2 { continue }
                let first = chunks.next().unwrap().to_vec();
                if chunks.all(|chunk| chunk.to_vec() == first) {
                    offset = o;
                    pattern_len = i;
                    break;
                }
            }
        }

        let mut max_lvl : i128 = lvl_diff.iter().take(offset as usize).map(|&x| x as i128).sum();

        let rep_height : i128 = lvl_diff[offset..(offset + pattern_len)]
            .to_vec()
            .into_iter()
            .map(|x| x as i128)
            .sum();
    
        let known_reps = (number_of_rocks - (offset + pattern_len) as i128) / pattern_len as i128;
        max_lvl += rep_height * (known_reps + 1);

    
        let remaining = (number_of_rocks - (offset + pattern_len) as i128) % pattern_len as i128;
        
        max_lvl += lvl_diff[offset..(offset+pattern_len)]
            .iter()
            .take(remaining as usize)
            .map(|&x| x as i128)
            .sum::<i128>();

    
        return max_lvl as i128;
    }

    return last_lvl as i128;
}

pub fn solve() -> Result<(), io::Error> {
    let raw_commands = get_lines("day_17").next().unwrap();
    let commands = raw_commands.chars().cycle();

    println!("First Star: {:?} ", simulate_n(commands.clone(), 2022));
    println!("Second Star: {:?} ", simulate_n(commands.clone(), 1_000_000_000_000));


    return Ok(())
}