use std::{ io, collections::{HashMap, HashSet} };
use itertools::Itertools;
use crate::utils::{self};

fn max_happiness(
    weights: &HashMap<(String,String), i32>, 
    friends: &HashSet<String>
) -> i32 {
    let mut best = i32::MIN;
    for perm in friends.iter().permutations(friends.len()) {
        let mut local_sum = 0;
        for i in 0..perm.len() {
            match i {
                x if x == perm.len() - 1 => {
                    let key = (perm[0].to_string(), perm[perm.len()-1].to_string());
                    local_sum += weights.get(&key).unwrap_or(&0);
                    let key2 = (perm[perm.len()-1].to_string(), perm[0].to_string());
                    local_sum += weights.get(&key2).unwrap_or(&0);
                },
                _ => {
                    let key = (perm[i].to_string(), perm[i+1].to_string());
                    local_sum += weights.get(&key).unwrap_or(&0);
                    let key2 = (perm[i+1].to_string(), perm[i].to_string());
                    local_sum += weights.get(&key2).unwrap_or(&0);
                }
            }
        }

        best = best.max(local_sum);
    }
    return best;
}

pub fn solve() -> Result<(), io::Error> {
    let lines = utils::get_lines("day_13");
    let mut weights : HashMap<(String,String), i32> = HashMap::new();
    let mut friends : HashSet<String> = HashSet::new();

    for line in lines { 
        let sign = if line.contains("lose") { -1 } else { 1 };

        let clean_line = line
            .replace(" would gain ", " ")
            .replace(" would lose ", " ")
            .replace(" happiness units by sitting next to ", " ")
            .replace(".", "");

        let v_line : Vec<&str> = clean_line.split(" ").collect();
        let val : i32 = v_line[1].parse().unwrap();
        friends.insert(v_line[0].to_string()); friends.insert(v_line[2].to_string());
        weights.insert((v_line[0].to_string(), v_line[2].to_string()), val * sign);
    }

    println!("First Star: {}", max_happiness(&weights, &friends));

    friends.insert("me".to_string());

    println!("Second Star: {}", max_happiness(&weights, &friends));

    return Ok(())
}