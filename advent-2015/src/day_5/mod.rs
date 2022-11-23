use std::{ io };
use std::str;
use crate::utils;

fn is_nice_str(lstr: &str) -> bool {
    let mut vowels_count = 0;
    for lchar in "aeiou".chars() {
        vowels_count += lstr.matches(lchar).count();
    }
    if vowels_count < 3 {  return false }

    for invalid in ["ab", "cd", "pq", "xy"] {
        if lstr.contains(invalid) {
            return false;
        }
    }

    let mut prev_char = lstr.as_bytes()[0] as char;
    let mut valid_repeat = false;
    for (idx,lchar) in lstr.chars().enumerate() {
        if idx != 0 {
            if prev_char == lchar { valid_repeat = true }
        }
        prev_char = lchar;
    }

    return true && valid_repeat;
}

fn is_nice_str2(lstr: &str) -> bool {
    let mut valid_pair = false;
    for idx in 0..lstr.len() {
        if idx + 1 < lstr.len() {
            let pair = format!("{}{}", lstr.as_bytes()[idx] as char, lstr.as_bytes()[idx+1] as char);
            if idx + 2 < lstr.len() {
                if lstr[idx+2..].contains(&pair) { valid_pair = true; }
            }
        }
    }

    if !valid_pair { return false }

    let mut valid_repeat = false;
    for idx in 0..lstr.len() {
        if idx + 2 < lstr.len() {
            if lstr.as_bytes()[idx] == lstr.as_bytes()[idx+2] {
                valid_repeat = true;
            }
        }
    }

    return true && valid_repeat;
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_5");
    let mut ns_count = 0;
    let mut ns_count2 = 0;

    for line in lines {
        if is_nice_str(&line) { ns_count += 1 }
        if is_nice_str2(&line) { ns_count2 += 1 }
    }
    println!("First Star: {}", ns_count);
    println!("Second Star: {}", ns_count2);


    return Ok(())
}