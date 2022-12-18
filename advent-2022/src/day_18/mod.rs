use std::{ io, collections::{HashMap, VecDeque, HashSet} };

use itertools::Itertools;

use crate::utils::{get_lines};

fn get_dist(c1 : (i128,i128,i128), c2 : (i128,i128,i128)) -> i128 {
    return (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs() + (c1.2 - c2.2).abs()
}

pub fn count_sides(
    coordinates: &Vec<(i128,i128,i128)>
) -> i128 {
    let mut res : i128 = 0;
    for &cube in coordinates {
        res += 6 - coordinates
            .iter()
            .filter(|&&cube2| get_dist(cube, cube2) == 1)
            .count() as i128;
    }
    return res;
}

pub fn is_free(
    air_cube: (i128,i128,i128),
    air_cubes: &HashSet<(i128,i128,i128)>,
    memo: &mut HashMap<(i128,i128,i128), bool>,
) -> bool {
    let dest = (-1,-1,-1);

    let mut queue : VecDeque<(i128,i128,i128)> = VecDeque::new();
    queue.push_back(air_cube);

    let mut visited : HashSet<(i128,i128,i128)> = HashSet::new();
    visited.insert(air_cube);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        if memo.contains_key(&curr) { return *memo.get(&curr).unwrap() }

        let possible_neighboors = [
            (curr.0+1,curr.1,curr.2), (curr.0,curr.1+1,curr.2), (curr.0,curr.1,curr.2+1),
            (curr.0-1,curr.1,curr.2), (curr.0,curr.1-1,curr.2), (curr.0,curr.1,curr.2-1),
        ];
        let neighboors = possible_neighboors.iter().filter(|n| air_cubes.contains(n)).collect_vec();

        for &neighboor in neighboors {
            if neighboor == dest {
                for v in visited {
                    memo.insert(v, true);
                }
                memo.insert(air_cube, true);
                return true 
            }
            if !visited.contains(&neighboor) {
                queue.push_back(neighboor);
                visited.insert(neighboor);
            }
        }
    }

    for v in visited {
        memo.insert(v, false);
    }
    return false;
}

pub fn count_exterior_sides(
    coordinates: &Vec<(i128,i128,i128)>
) -> i128 {
    let mut res : i128 = 0;
    let max_x = coordinates.iter().map(|c| c.0).max().unwrap() + 1;
    let max_y = coordinates.iter().map(|c| c.1).max().unwrap() + 1;
    let max_z = coordinates.iter().map(|c| c.2).max().unwrap() + 1;
    
    let coordinates_set : HashSet<(i128,i128,i128)> = coordinates.iter().map(|&c| c).collect();

    let mut air_cubes : HashSet<(i128,i128,i128)> = HashSet::new();
    for x in -1..max_x+1 { for y in -1..max_y+1 { for z in -1..max_z+1 { 
        if !coordinates_set.contains(&(x,y,z)) {
            air_cubes.insert((x,y,z));
        }
    }}}

    let mut memo : HashMap<(i128,i128,i128), bool> = HashMap::new();
    for air in air_cubes.clone() {
        if !is_free(air, &air_cubes, &mut memo) { continue }
        res += coordinates
            .iter()
            .filter(|&&cube| get_dist(air, cube) == 1)
            .count() as i128;
    }

    return res;
}

pub fn solve() -> Result<(), io::Error> {
    let coordinates = get_lines("day_18")
        .map(|x| {
            let (x,z,y) = x.split(",").map(|n| n.parse::<i128>().unwrap()).collect_tuple().unwrap();
            return (x,y,z);
        }).collect_vec();
    
    
    println!("First Star: {:?} ", count_sides(&coordinates));
    println!("Second Star: {:?} ", count_exterior_sides(&coordinates));

    return Ok(())
}