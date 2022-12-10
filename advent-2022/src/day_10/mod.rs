use std::{ io };

use itertools::Itertools;

use crate::utils::{self};

fn get_interesting_signals(commands: &Vec<String>) -> i32 {
    let interesting_cycles : [i32;6] = [20,60,100,140,180,220];
    let mut res = 0;

    let mut cycle = 0;
    let mut x = 1;

    let mut curr : usize = 0;

    for cmd in commands {
        if curr >= interesting_cycles.len() { break }
        match cmd.split_whitespace().collect_vec()[..] {
            ["noop"] => {
                if (cycle..cycle+2).contains(&interesting_cycles[curr]) { 
                    res += interesting_cycles[curr] * x; curr += 1;
                }
                cycle += 1;
            },
            ["addx", val] => {
                if (cycle..cycle+3).contains(&interesting_cycles[curr]) { 
                    res += interesting_cycles[curr] * x;  curr += 1;
                }
                cycle += 2;
                x += val.parse::<i32>().unwrap();
            }
            _ => unreachable!()
        }
    }
    return res;
}

fn print_on_cmd(mut cycle: i32, cycle_len: i32, sprite_pos: i32) {
    if cycle > 39 { cycle -= 40 * ((cycle+1) / 40) }
    for cycle_it in cycle..cycle + cycle_len {
        if (sprite_pos..sprite_pos+3).contains(&cycle_it) { print!("#") }
        else { print!(".") }
        if (cycle_it + 1) % 40 == 0 { println!() }
    }
}

fn draw_signal(commands: &Vec<String>) {
    let mut cycle = 0;
    let mut sprite_pos = 0;

    for cmd in commands {
        match cmd.split_whitespace().collect_vec()[..] {
            ["noop"] => {
                print_on_cmd(cycle, 1, sprite_pos);
                cycle += 1;
            },
            ["addx", val] => {
                print_on_cmd(cycle, 2, sprite_pos);
                cycle += 2;
                sprite_pos += val.parse::<i32>().unwrap();
            }
            _ => unreachable!()
        }
    }
}

pub fn solve() -> Result<(), io::Error> {
    let commands : Vec<String> = utils::get_lines("day_10").collect();

    println!("First Star: {:?}", get_interesting_signals(&commands));
    println!("Second Star:");
    draw_signal(&commands);

    return Ok(())
}