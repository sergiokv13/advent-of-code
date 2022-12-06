use std::{ io, collections::HashSet };

use crate::utils;

fn get_first_marker(signal: &str, marker_len: usize) -> usize {
    for i in 0..signal.len()-marker_len {
        let chars_set : HashSet<char> = signal[i..i+marker_len].chars().collect();
        if chars_set.len() == marker_len { return i + marker_len }
    }
    return 0;
}

pub fn solve() -> Result<(), io::Error>{
    let mut lines = utils::get_lines("day_6");
    let signal = lines.next().unwrap();
    
    println!("First Star: {:?}",get_first_marker(&signal, 4));
    println!("First Star: {:?}",get_first_marker(&signal, 14));

    return Ok(())
}