use std::{ io, cmp::Ordering };
use itertools::Itertools;

use crate::utils::{get_lines};

fn compare(p1 : &serde_json::Value, p2 : &serde_json::Value) -> Ordering {
    if p1.is_number() && p2.is_number() {
        if p1.as_u64().unwrap() < p2.as_u64().unwrap() { return Ordering::Less }
        else if p1.as_u64().unwrap() > p2.as_u64().unwrap() { return Ordering::Greater }
        else { return Ordering::Equal }
    }

    if p1.is_array() || p2.is_array() {
        let p1_wrapped = Vec::from([p1.clone()]);
        let p1_arr = if p1.is_array() { p1.as_array().unwrap() } else { &p1_wrapped };
        let p2_wrapped = Vec::from([p2.clone()]);
        let p2_arr = if p2.is_array() { p2.as_array().unwrap() } else { &p2_wrapped };

        for i in 0..p1_arr.len() {
            if i >= p2_arr.len() { return Ordering::Greater }
            let compared = compare(&p1_arr[i], &p2_arr[i]);
            if compared == Ordering::Less { return Ordering::Less }
            else if compared == Ordering::Greater { return Ordering::Greater }
        }
        if p1_arr.len() < p2_arr.len() { return Ordering::Less }
    }
    return Ordering::Equal;
}

pub fn solve() -> Result<(), io::Error> {
    let lines = get_lines("day_13").collect_vec();
    let mut sum = 0;

    let mut packets : Vec<(serde_json::Value, bool)> = Vec::from([
        (serde_json::from_str("[[2]]").unwrap(), true),
        (serde_json::from_str("[[6]]").unwrap(), true),
    ]);

    for (idx, chunk) in lines.chunks(3).into_iter().enumerate() {
        let p1: serde_json::Value = serde_json::from_str(&chunk[0]).unwrap();
        let p2: serde_json::Value = serde_json::from_str(&chunk[1]).unwrap();

        packets.push((p1.clone(), false));
        packets.push((p2.clone(), false));

        if compare(&p1, &p2) == Ordering::Less {
            sum += idx + 1;
        }
    }

    packets.sort_by(|p1, p2| compare(&p1.0, &p2.0));
    
    let sorted_mult = packets
        .iter()
        .enumerate()
        .map(|(idx, p)| if p.1 { idx as i64 + 1} else { 1 as i64 })
        .reduce(|prev, val| prev * val).unwrap();

    println!("First Star: {:?}", sum);
    println!("Second Star: {:?}", sorted_mult);

    return Ok(())
}