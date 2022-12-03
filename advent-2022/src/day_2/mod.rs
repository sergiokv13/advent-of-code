use std::{ io, collections::HashMap };

use crate::utils;

fn get_score_1(
    strategy : &Vec<(char,char)>,
    shape_points : &HashMap<char, i32>,
) -> i32 {
    return  strategy.iter().map(|game| {
        match game {
            x if x.0 == x.1 => shape_points.get(&game.1).unwrap() + 3,
            x if (
                (x.0 == 'R' && x.1 == 'P') || (x.0 == 'S' && x.1 == 'R') || (x.0 == 'P' && x.1 == 'S')
             ) => shape_points.get(&game.1).unwrap() + 6,
            _ => shape_points.get(&game.1).unwrap() + 0,
        }
    }).sum();
}

fn get_score_2(
    strategy : &Vec<(char,char)>,
    shape_points : &HashMap<char, i32>,
) -> i32 {
    return strategy.iter().map(|game| {
        match game.1 {
            // lose
            'X' => {
                match game.0 {
                    'R' => *shape_points.get(&'S').unwrap(),
                    'S' => *shape_points.get(&'P').unwrap(),
                    _ => *shape_points.get(&'R').unwrap(),

                }
            },
            // win
            'Z' => {
                6 + {match game.0 {
                    'R' => *shape_points.get(&'P').unwrap(),
                    'P' => *shape_points.get(&'S').unwrap(),
                    _ => *shape_points.get(&'R').unwrap(),
                }}
            },
            // draw
            _ => shape_points.get(&game.0).unwrap() + 3,
        }
    }).sum();
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_2");
    let shape_points : HashMap<char, i32> = HashMap::from([('R',1),('P',2),('S',3)]);
    let u1_map: HashMap<char, char> = HashMap::from([('A','R'),('B','P'),('C','S')]);
    let u2_map: HashMap<char, char> = HashMap::from([('X','R'),('Y','P'),('Z','S')]);

    let mut strategy1 : Vec<(char,char)> = Vec::new();
    let mut strategy2 : Vec<(char,char)> = Vec::new();

    for line in lines {
        let v_line : Vec<char> = line
            .split(" ")
            .map(|x| x.chars().nth(0).unwrap())
            .collect();

        strategy1.push((*u1_map.get(&v_line[0]).unwrap(), *u2_map.get(&v_line[1]).unwrap()));
        strategy2.push((*u1_map.get(&v_line[0]).unwrap(), v_line[1]));
    }
    
    println!("First Star: {}", get_score_1(&strategy1, &shape_points));
    println!("Second Star: {}", get_score_2(&strategy2, &shape_points));

    return Ok(())
}