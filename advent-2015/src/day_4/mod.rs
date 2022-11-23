use std::{ io };
use std::str;
use crate::utils;

fn get_number(key: &str, start: &str) -> String {
    let mut counter = 1;

    loop {
        let hash_str = format!("{}{}", key, counter);
        let digest = md5::compute(hash_str);

        if format!("{:x}", digest).starts_with(start) {
            break
        }
        counter += 1;
    }

    return format!("{}", counter);
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_4");
    for line in lines {
        println!("First Star: {}", get_number(&line, "00000"));
        println!("Second Star: {}", get_number(&line, "000000"));
    }

    return Ok(())
}