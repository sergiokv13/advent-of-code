use std::{ io };

use itertools::Itertools;

use crate::utils;

fn get_next_char(curr_char : char) -> (char, i32) {
    if curr_char == 'z' {
        ('a', 1)
    } else {
        ((curr_char as u8 + 1) as char, 0)
    }
}

fn get_next_unverified_str(curr_str: &mut Vec<char>)  {
    for i in (0..curr_str.len()).rev() {
        let (next_char, next_carry) = get_next_char(curr_str[i]);
        curr_str[i] = next_char;

        if next_carry == 0 {
            break;
        }
    }
}

fn validate_valid_chars(curr_str: &Vec<char>) -> bool {
    for el in curr_str {
        if *el == 'i' || *el == 'o' || *el == 'l' {
            return false
        }
    }
    return true;
}

fn validate_valid_seq(curr_str: &Vec<char>) -> bool {
    let mut curr_char: Option<char> = None;
    let mut max_size = 0;

    for el in curr_str {
        if curr_char != None {
            if (curr_char.unwrap() as u8 + 1) as char == *el {
                max_size += 1;
                if max_size == 2 {
                    return true;
                }
            } else {
                max_size = 0;
            }
        }
        curr_char = Some(*el);
    }
    return false;
}

fn validate_valid_pairs(curr_str: &Vec<char>) -> bool {
    let mut first_pair_char : Option<char> = None;
    let mut prev_el : Option<char> = None;

    for el in curr_str {
        if prev_el != None && *el == prev_el.unwrap() {
            // We just found a pair
            if first_pair_char == None {
                first_pair_char = Some(*el);
            } else {
                if first_pair_char != prev_el {
                    return true;
                }
            }
        }

        prev_el = Some(*el);
    }

    return false;
}

fn get_next_str(curr_str: &mut Vec<char>){
    loop {
        get_next_unverified_str(curr_str);
        if 
            validate_valid_chars(curr_str) &&
            validate_valid_pairs(curr_str) &&
            validate_valid_seq(curr_str) {
                break;
            }
    }
}

pub fn solve() -> Result<(), io::Error> {
    let lines = utils::get_lines("day_11");
    let mut _val = String::from("");

    for line in lines { 
        _val = line;
        unsafe {
            let mut v : Vec<char> = _val.as_mut_vec().iter().map(|x| *x as char).collect_vec();
            get_next_str(&mut v);
            println!("First Star: {:?} ", String::from_iter(v.iter()));
            get_next_str(&mut v);
            println!("Second Star: {:?} ", String::from_iter(v.iter()));
        }
    }

    return Ok(())
}