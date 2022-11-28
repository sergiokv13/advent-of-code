use std::{ io };
use crate::utils::{self};

fn encode(ltxt: &str) -> String {
    let mut rs : String = ltxt.to_string();

    if rs.contains("\\\"") { rs = rs.replace("\\\"", "_") }
    if rs.contains("\\\\") {  rs = rs.replace("\\\\", "_") }
    if rs.contains("\\x") {
        let idxs: Vec<_> = rs.match_indices("\\x").collect();
        let mut to_rep: Vec<String> = Vec::new();

        for idx in idxs {
            let tr = &rs[idx.0..idx.0+4];
            to_rep.push(tr.to_string());
        }

        for tr in to_rep {
            rs = rs.replace(&tr, "_");
        }
    }

    return rs;
}

fn decode(ltxt: &str) -> String {
    let mut rs : String = ltxt.to_string();

    if rs.contains("\"") { rs = rs.replace("\"", "__") }
    if rs.contains("\\") { rs = rs.replace("\\", "__") }
    if rs.contains("\\x") { rs = rs.replace("\\x", "___") }

    return rs;
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_8");
    let mut count = 0;
    let mut escaped_count = 0;
    let mut augmented_count = 0;

    for line in lines {
        count += line.len();
        escaped_count += encode(&line).len() - 2; // Removing 2 due to " wrapping the str
        augmented_count += decode(&line).len() + 2; // Adding 2 due to " wrapping the str
    }
    println!("First Star: {}", count - escaped_count);
    println!("Second Star: {}", augmented_count - count);

    return Ok(())
}