use std::{ io };
use serde_json;
use crate::utils;

fn get_numbers_sum(raw_json: serde_json::Value, ignore_red : bool) -> i64 {
    // If we find a number, then we return it
    if raw_json.is_number() {
        return raw_json.as_i64().unwrap();
    }

    let mut sum = 0;
    // If it's an array, we will go trough all the elements
    if raw_json.is_array() {
        for el in raw_json.as_array().unwrap() {
            sum += get_numbers_sum(el.clone(), ignore_red);
        }
    } else if raw_json.is_object() {
        for el in raw_json.as_object().unwrap().values() {
            sum += get_numbers_sum(el.clone(), ignore_red);

            if ignore_red && el == "red" {
                return 0;
            }
        }
    }

    return sum;
}

pub fn solve() -> Result<(), io::Error> {
    let lines = utils::get_lines("day_12");
    let mut _val = String::from("");

    let mut global_sum = 0;
    let mut global_sum2 = 0;

    for line in lines { 
        let v: serde_json::Value = serde_json::from_str(&line)?;
        global_sum += get_numbers_sum(v.clone(), false);
        global_sum2 += get_numbers_sum(v, true);
    }

    println!("First Star: {}", global_sum);
    println!("Second Star: {}", global_sum2);

    return Ok(())
}