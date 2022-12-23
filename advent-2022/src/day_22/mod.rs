use std::{ io, collections::HashMap };

use itertools::Itertools;

use crate::utils::{get_lines};

// For now we will take in considaration only 2
enum CubeConfiguration {
    Flat,
    //    1
    //  234
    //    56
    _First,
    //    12
    //    3
    //   45
    //   6
    Second,
}

fn rotate(direction: char, command: char) -> char {
    match (direction, command) {
        ('>', 'R') => return 'v',
        ('v', 'R') => return '<',
        ('<', 'R') => return '^',
        ('^', 'R') => return '>',
        ('^', 'L') => return '<',
        ('<', 'L') => return 'v',
        ('v', 'L') => return '>',
        ('>', 'L') => return '^',
        _ => unreachable!()
    }
}

// Cubes need to be converted to first configuration for this step
fn _wrap_cube_first(
    curr_pos: (usize, usize),
    curr_cube: usize,
    size: usize,
    direction: char
) -> (usize, char, (usize, usize)) {
    let (row, col) = curr_pos;
    match (curr_cube, direction) {
        (1,'>') => (6,'<',(size-row-1, size-1)),
        (1,'^') => (2,'v',(0, size-col-1)),
        (1,'<') => (3,'v', (0, row)),
        (1,'v') => (4,'v',(0, col)),
    
        (2,'>') => (3,'>',(row, 0)),
        (2,'^') => (1,'v',(0, size-col-1)),
        (2,'<') => (6,'^', (size-1, size-row-1)),
        (2,'v') => (5,'^',(size-1, size-col-1)),
    
        (3,'>') => (4,'>',(row, 0)),
        (3,'^') => (1,'>',(col, 0)),
        (3,'<') => (2,'<', (row, size-1)),
        (3,'v') => (5,'>',(size-col-1, 0)),
    
        (4,'>') => (6,'v',(0, size - row - 1)),
        (4,'^') => (1,'^',(size-1, col)),
        (4,'<') => (3,'<', (row, size-1)),
        (4,'v') => (5,'v',(0, col)),
    
        (5,'>') => (6,'>',(row, 0)),
        (5,'^') => (4,'^',(size-1, col)),
        (5,'<') => (3,'^', (size-1, size-row-1)),
        (5,'v') => (2,'^',(size-1, size-col-1)),
    
        (6,'>') => (1,'<',(size-row-1, size-1)),
        (6,'^') => (4,'<',(size-col-1, size-1)),
        (6,'<') => (5,'<', (row, size-1)),
        (6,'v') => (2,'>',(size-col-1, 0)),

        _ => unreachable!(),
    }
}

fn wrap_cube_second(
    curr_pos: (usize, usize),
    curr_cube: usize,
    size: usize,
    direction: char
) -> (usize, char, (usize, usize)) {
    let (row, col) = curr_pos;
    match (curr_cube, direction) {
        (1,'>') => (2,'>',(row, 0)),
        (1,'^') => (6,'>',(col, 0)),
        (1,'<') => (4,'>',(size-row-1, 0)),
        (1,'v') => (3,'v',(0, col)),
    
        (2,'>') => (5,'<',(size-row-1, size-1)),
        (2,'^') => (6,'^',(size-1, col)),
        (2,'<') => (1,'<', (row, size-1)),
        (2,'v') => (3,'<',(col, size-1)),
    
        (3,'>') => (2,'^', (size-1, row)),
        (3,'^') => (1,'^',(size-1, col)),
        (3,'<') => (4,'v',(0, row)),
        (3,'v') => (5,'v',(0, col)),
    
        (4,'>') => (5,'>',(row, 0)),
        (4,'^') => (3,'>',(col, 0)),
        (4,'<') => (1,'>', (size-row-1, 0)),
        (4,'v') => (6,'v',(0, col)),
    
        (5,'>') => (2,'<',(size-row-1, size-1)),
        (5,'^') => (3,'^',(size-1, col)),
        (5,'<') => (4,'<', (row, size-1)),
        (5,'v') => (6,'<',(col, size-1)),
    
        (6,'>') => (5,'^',(size-1, row)),
        (6,'^') => (4,'^',(size-1, col)),
        (6,'<') => (1,'v', (0, row)),
        (6,'v') => (2,'v',(0, col)),

        _ => unreachable!(),
    }
}

fn get_next_pos_cube(
    curr_pos: (usize, usize), 
    curr_cube: usize,
    direction: char, 
    cubes: &HashMap<usize, Vec<Vec<char>>>,
) -> (usize, char, (usize, usize)) {
    let (row, col) = curr_pos;
    let size = cubes.get(&1).unwrap().len();
    match direction {
        '>' => if col + 1 < size { return (curr_cube, direction, (row, col + 1))} else { return wrap_cube_second(curr_pos, curr_cube, size, direction) },
        '<' => if col as i32 - 1 >= 0 { return (curr_cube, direction, (row, col - 1)) } else { return wrap_cube_second(curr_pos, curr_cube, size, direction) },
        'v' => if row + 1 < size { return (curr_cube, direction, (row + 1, col))} else { return wrap_cube_second(curr_pos, curr_cube, size, direction) },
        '^' => if row as i32 - 1 >= 0 { return (curr_cube, direction, (row - 1, col));} else { return wrap_cube_second(curr_pos, curr_cube, size, direction) },
        _ => unreachable!()
    }
}

fn move_one_step_cube(
    curr_pos: (usize, usize), 
    curr_cube: usize,
    direction: char, 
    cubes: &HashMap<usize, Vec<Vec<char>>>,
) -> (usize, char, (usize, usize)) {
    let (cand_cube, cand_dir, cand_pos) = get_next_pos_cube(curr_pos, curr_cube, direction, cubes);    
    let candidate_value = cubes.get(&cand_cube).unwrap()[cand_pos.0][cand_pos.1];
    match candidate_value {
        '.' => return (cand_cube, cand_dir, cand_pos),
        '#' => return (curr_cube, direction, curr_pos),
        _ => unreachable!()
    }
}

fn grid_to_cube(grid: &Vec<Vec<char>>) -> HashMap<usize, Vec<Vec<char>>> {
    let size = grid
        .iter()
        .map(|row| row.into_iter().filter(|&el| *el != 'r').count())
        .min().unwrap();
    
    let mut cubes : HashMap<usize, Vec<Vec<char>>> = HashMap::new();
    let mut curr_cube_idx= 1;

    for grid_chunk in grid.iter().chunks(size).into_iter() {
        let cleaned_chunk = grid_chunk
            .map(|row| 
                row.into_iter()
                .filter(|&el| *el != 'r')
                .map(|c| *c).collect_vec()
            )
            .collect_vec();

        // println!("chunk: ");
        // for row in cleaned_chunk.clone() {
        //     println!("{:?}", row);
        // }

        let cubes_in_chunk = cleaned_chunk[0].len() / size;    
        for i in 0..cubes_in_chunk {
            let curr_cube = cleaned_chunk.clone()
                .into_iter()
                .map(|row| row[i*size..((i+1)*size)]
                    .into_iter().map(|c| *c).collect_vec()    
                )
                .collect_vec();

            cubes.insert(curr_cube_idx, curr_cube);
            curr_cube_idx += 1;
        }
    }
    return cubes;
}


fn simulate_cube(
    grid: &Vec<Vec<char>>, 
    commands: Vec<String>,
    configuration: CubeConfiguration,
) -> ((usize, usize), char) {
    let cubes = grid_to_cube(grid);

    let size = cubes.get(&1).unwrap()[0].len();

    // find initial pos
    let mut curr_pos = (0,0);
    let mut curr_cube = 1;
    let mut curr_dir = '>';

    for command in commands {
        match command.parse::<i32>() {
            Ok(v) => {
                for _i in 0..v {
                    (curr_cube, curr_dir, curr_pos) = move_one_step_cube(curr_pos, curr_cube, curr_dir, &cubes);
                }
            },
            _ => curr_dir = rotate(curr_dir, command.chars().next().unwrap())
        }
    }

    match configuration {
        CubeConfiguration::_First => {
            match curr_cube {
                1 => return ((curr_pos.0, curr_pos.1 + size * 2), curr_dir),
                
                2 => return ((curr_pos.0 + size, curr_pos.1), curr_dir),
                3 => return ((curr_pos.0 + size, curr_pos.1 + size), curr_dir),
                4 => return ((curr_pos.0 + size, curr_pos.1 + size * 2), curr_dir),
                
                5 => return ((curr_pos.0 + size * 2, curr_pos.1 + size * 2), curr_dir),
                6 => return ((curr_pos.0 + size * 2, curr_pos.1 + size * 3), curr_dir),
                _ => unreachable!()
            }
        },
        CubeConfiguration::Second => {
            match curr_cube {
                1 => return ((curr_pos.0, curr_pos.1 + size), curr_dir),
                2 => return ((curr_pos.0, curr_pos.1 + size * 2), curr_dir),

                3 => return ((curr_pos.0 + size, curr_pos.1 + size), curr_dir),
                
                4 => return ((curr_pos.0 + size * 2, curr_pos.1), curr_dir),
                5 => return ((curr_pos.0 + size * 2, curr_pos.1 + size), curr_dir),
                
                6 => return ((curr_pos.0 + size * 3, curr_pos.1), curr_dir),
                _ => unreachable!()
            }
        },
        _ => unreachable!()
    }
}

fn get_next_pos_flat(curr_pos: (usize, usize), direction: char, grid: &Vec<Vec<char>>) -> (usize, usize) {
    let (row, col) = curr_pos;
    let col_size = grid[0].len();
    let row_size = grid.len();
    match direction {
        '>' => if col + 1 < col_size { (row, col + 1)} else { (row, 0) },
        '<' => if col as i32 - 1 >= 0 { (row, col - 1)} else { (row, col_size-1) },
        'v' => if row + 1 < row_size { (row + 1, col)} else { (0, col) },
        '^' => if row as i32 - 1 >= 0 { (row - 1, col)} else { (row_size-1, col) },
        _ => unreachable!()
    }
}

fn move_one_step_flat(curr_pos: (usize, usize), direction: char, grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut candidate = curr_pos;
    let mut candidate_value = 'r';
    while candidate_value == 'r' {
        candidate = get_next_pos_flat(candidate, direction, grid);
        candidate_value = grid[candidate.0][candidate.1];
        match candidate_value {
            '.' => return candidate,
            '#' => return curr_pos,
            _ => continue,
        }
    }
    return curr_pos;
}

fn simulate_flat(grid: &Vec<Vec<char>>, commands: Vec<String>) -> ((usize, usize), char) {
    // find initial pos
    let mut curr_pos = (0,0);
    let mut curr_dir = '>';
    for col in 0..grid[0].len() {
        if grid[0][col] == '.' {
            curr_pos = (0,col);
            break;
        }
    }
    
    for command in commands {
        match command.parse::<i32>() {
            Ok(v) => {
                for _i in 0..v {
                    curr_pos = move_one_step_flat(curr_pos, curr_dir, grid);
                }
            },
            _ => curr_dir = rotate(curr_dir, command.chars().next().unwrap())
        }
    }

    return (curr_pos, curr_dir);
}

fn get_password(grid: &Vec<Vec<char>>, commands: Vec<String>, configuration: CubeConfiguration) -> i128 {
    let (mut _final_pos, mut _dir) = ((0,0), '>');
    match configuration {
        CubeConfiguration::Flat => (_final_pos, _dir) = simulate_flat(grid, commands),
        _ => (_final_pos, _dir) = simulate_cube(grid, commands, configuration),
    }

    _final_pos = (_final_pos.0 + 1, _final_pos.1 + 1);

    match _dir {
        '>' => (0 + 1000 * _final_pos.0 + 4 * _final_pos.1) as i128,
        '<' => (2 + 1000 * _final_pos.0 + 4 * _final_pos.1) as i128,
        'v' => (1 + 1000 * _final_pos.0 + 4 * _final_pos.1) as i128,
        '^' => (3 + 1000 * _final_pos.0 + 4 * _final_pos.1) as i128,
        _ => unreachable!()
    }
}

pub fn solve() -> Result<(), io::Error> {
    let mut grid : Vec<Vec<char>> = Vec::new();
    
    let mut reading_grid = true;
    let mut raw_commands : String = String::from("");
    for line in get_lines("day_22") {
        if line.is_empty() { reading_grid = false};
        if reading_grid { grid.push(line.replace(" ", "r").chars().collect_vec()) }
        if !reading_grid { raw_commands = line.clone() }
    }

    // fill grid with r for empty spaces
    let max_size = grid.iter().map(|r| r.len()).max().unwrap();
    for i in 0..grid.len() {
        while grid[i].len() < max_size {
            grid[i].push('r');
        }
    }

    let mut commands : Vec<String> = Vec::new();
    // parse raw commands to separate numbers from letters
    let mut curr_str = String::from("");
    for el in raw_commands.chars() {
        if el.is_numeric() { curr_str.push(el) }
        else { 
            if !curr_str.is_empty() { 
                commands.push(curr_str) ;
                curr_str = "".to_string();
            }
            commands.push(el.to_string());
        }
    }
    if !curr_str.is_empty() { commands.push(curr_str) }

    println!("First Star: {:?} ", get_password(&grid, commands.clone(), CubeConfiguration::Flat));
    println!("Second Star: {:?} ", get_password(&grid, commands.clone(), CubeConfiguration::Second));

    return Ok(())
}