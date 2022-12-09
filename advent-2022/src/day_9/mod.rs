use std::{ io, collections::{HashMap, HashSet} };

use crate::utils::{self};

// return new position and a boolean if the tail moved or not
fn move_tail(tp: &mut (i32,i32), hp: &(i32,i32)) -> bool {
    match tp {
        _x if tp == hp => return false,
        _x if (tp.0 - hp.0).abs() <= 1 && (tp.1 - hp.1).abs() <= 1 => return false,
        _ => {
            if tp.0 != hp.0 { 
                tp.0 = if hp.0 > tp.0 { tp.0 + 1 } else { tp.0 - 1 };
            }
            if tp.1 != hp.1 { 
                tp.1 = if hp.1 > tp.1 { tp.1 + 1 } else { tp.1- 1 };
            }
            return true;
        }
    }
}

fn perform_move(
    movecmd: String,
    knot_pos: &mut  HashMap<i32,(i32,i32)>,
    knots: i32,
    visited: &mut HashSet<(i32,i32)>, 
) {
    let mut row_upd = 0;let mut col_upd = 0;
    let val: i32;
    match movecmd.split_whitespace().collect::<Vec<_>>()[..] {
        ["R", raw_val] => { val = raw_val.parse::<i32>().unwrap(); col_upd = 1;}
        ["L", raw_val] => { val = raw_val.parse::<i32>().unwrap(); col_upd = -1;}
        ["U", raw_val] => { val = raw_val.parse::<i32>().unwrap(); row_upd = -1;}
        ["D", raw_val] => { val = raw_val.parse::<i32>().unwrap(); row_upd = 1;}
        _ => unreachable!()
    }

    for _i in 0..val {
        let mut head = *knot_pos.get(&0).unwrap();
        head.0 += row_upd; head.1 += col_upd;
        knot_pos.insert(0, head);
    
        for knot in 0..knots {
            let hp = *knot_pos.get(&knot).unwrap_or(&(0,0));
            let mut tp = *knot_pos.get(&(knot+1)).unwrap_or(&(0,0));

            let moved = move_tail(&mut tp, &hp);
            if moved {
                knot_pos.insert(knot+1, tp);
                let _ = knot + 1 == knots && visited.insert(tp);
            }

        }
    }
}

fn get_visited(moves: &Vec<String>, knots: i32) -> i32 {
    let mut visited : HashSet<(i32,i32)> = HashSet::new();
    visited.insert((0,0));

    let mut knot_pos : HashMap<i32,(i32,i32)> = HashMap::new();
    knot_pos.insert(0, (0,0)); // head

    for movecmd in moves {
        perform_move(movecmd.to_string(), &mut knot_pos, knots, &mut visited)
    }
    return visited.len() as i32;
}

pub fn solve() -> Result<(), io::Error> {
    let moves : Vec<String> = utils::get_lines("day_9").collect();

    println!("First Star: {:?}", get_visited(&moves,1));
    println!("Second Star: {:?}", get_visited(&moves,9));
    return Ok(())
}