use std::{ io, collections::HashMap };

use itertools::Itertools;

use crate::utils::{get_lines};

fn get_monkey_value(
    monkeys: &HashMap<String,String>, 
    memo: &mut HashMap<String, i128>,
    curr_monkey: String
) -> i128 {
    if memo.contains_key(&curr_monkey) {
        return *memo.get(&curr_monkey).unwrap();
    }

    let mut _res = 0;
    
    let operation = monkeys.get(&curr_monkey).unwrap().to_string();
    let op_splitted = operation.split(" ").collect_vec();

    match op_splitted.len() {
        1 => _res = op_splitted[0].parse::<i128>().unwrap(),
        _ => {
            let (m1, op, m2) = op_splitted.iter().collect_tuple().unwrap();
            let nm1 = get_monkey_value(monkeys, memo, m1.to_string());
            let nm2 = get_monkey_value(monkeys, memo, m2.to_string());

            match *op {
                "+" => _res = nm1 + nm2,
                "-" => _res = nm1 - nm2,
                "*" => _res = nm1 * nm2,
                "/" => _res = nm1 / nm2,
                _ => _res = 0
            }
        }   
    }
    memo.insert(curr_monkey, _res);
    return _res;
}

fn get_humn_yell(monkeys: &HashMap<String,String>) -> i128 {
    let monkeys = monkeys.clone();
    
    let (m1,_,m2) = monkeys.get("root").unwrap().split_whitespace().collect_tuple().unwrap();

    // this value remains always the same, so it's the target
    let target = (|| {
        let mut monkeys = monkeys.clone();
        monkeys.insert("humn".to_string(), format!("{:?}", 0));

        let mut memo : HashMap<String, i128> = HashMap::new();
        return get_monkey_value(&monkeys, &mut memo, m2.to_string());
    })();

    let direction = (|| {
        let mut monkeys = monkeys.clone();

        monkeys.insert("humn".to_string(), format!("{:?}", 1));

        let mut memo : HashMap<String, i128> = HashMap::new();
        let nm1 = get_monkey_value(&monkeys, &mut memo, m1.to_string());

        monkeys.insert("humn".to_string(), format!("{:?}", 100));

        let mut memo : HashMap<String, i128> = HashMap::new();
        let nm1_2 = get_monkey_value(&monkeys, &mut memo, m1.to_string());

        return if nm1 < nm1_2 {'<'} else {'>'}
    })();

    let mut humn_range = [0, 5_000_000_000_000];

    // Will do a binary search to find the nearest val
    let mut nearest_val = (|| {
        loop {
            let humn_val = (humn_range[0] + humn_range[1]) / 2;
            if humn_range[0] >= humn_range[1] { return humn_range[0] }

            let mut monkeys = monkeys.clone();
            monkeys.insert("humn".to_string(), format!("{:?}", humn_val));
        
            let mut memo : HashMap<String, i128> = HashMap::new();
            let nm1 = get_monkey_value(&monkeys, &mut memo, m1.to_string());

            if nm1 == target { return humn_val }
            
            match direction {
                '>' => {
                    if nm1 < target {
                        humn_range = [humn_range[0]+1, humn_val];
                    } else {
                        humn_range = [humn_val, humn_range[1]-1];
                    } 
                },
                '<' => {
                    if nm1 > target {
                        humn_range = [humn_range[0]+1, humn_val];
                    } else {
                        humn_range = [humn_val, humn_range[1]-1];
                    }
                },
                _ => unreachable!(),
            }
        }
    })();

    // Find the first ocurrence in which we were able to get into the number (should be near)
    loop {
        let mut monkeys = monkeys.clone();
        monkeys.insert("humn".to_string(), format!("{:?}", nearest_val));

        let mut memo : HashMap<String, i128> = HashMap::new();
        let number = get_monkey_value(&monkeys, &mut memo, m1.to_string());

        if number != target { return nearest_val + 1 }
        nearest_val -= 1;
    }
}

pub fn solve() -> Result<(), io::Error> {
    let monkeys : HashMap<String,String> = get_lines("day_21")
        .map(|x| x.split_once(": ")
            .map(|(m1, m2)| (m1.to_string(), m2.to_string()))
            .unwrap()
        ).collect();

    let mut memo : HashMap<String, i128> = HashMap::new();
    println!("First Star: {:?} ", get_monkey_value(&monkeys, &mut memo, "root".to_string()));
    println!("Second Star: {:?} ", get_humn_yell(&monkeys));

    return Ok(())
}