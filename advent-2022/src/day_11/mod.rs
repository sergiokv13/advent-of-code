use std::{ io, cell::RefCell, collections::HashMap };
use eval::{eval};
use itertools::Itertools;

use crate::utils;

#[derive(Debug, Clone)]
struct Monkey {
    items: RefCell<Vec<u128>>,
    operation: String,
    test_div: u128,
    test_cond: (usize,usize),
}

impl Monkey {
    pub fn create(operation: String, test_div: u128, test_cond: (usize, usize), items : Vec<u128>)-> Monkey {
        let m = Monkey {
            operation: operation,
            test_div: test_div, 
            test_cond: test_cond,
            items: RefCell::new(Vec::new()) 
        };
        for v in items { m.items.borrow_mut().push(v) }
        m
    }

    pub fn throw(&self, monkeys: &Vec<Monkey>, with_div: bool, common_denom: u128) {  
        let old = self.items.borrow_mut()[0];
        let op : &str = &self.operation.replace("old", &old.to_string());
        let mut new : u128 = eval(op).unwrap().as_i64().unwrap() as u128;

        self.items.borrow_mut().remove(0);
        if with_div {
            new = new / 3;
        } else {
            new = new % common_denom;
        }

        if new % self.test_div == 0 {
            monkeys[self.test_cond.0].items.borrow_mut().push(new);
        } else {
            monkeys[self.test_cond.1].items.borrow_mut().push(new);
        }
    }
}

fn simulate_round(
    monkeys: &Vec<Monkey>, 
    inspected: &mut HashMap<usize, u128>, 
    with_div: bool, 
    common_denom: u128
) {
    for (mid, monkey) in monkeys.iter().enumerate() {
        *inspected.entry(mid).or_insert(0) += monkey.items.borrow().len() as u128;

        while !monkey.items.borrow().is_empty() {
            monkey.throw(&monkeys, with_div, common_denom);
        }
    }
}

fn first_star(monkeys: &Vec<Monkey>) -> u128 {
    let mut inspected: HashMap<usize, u128> = HashMap::new();

    for _i in 0..20 {
        simulate_round(monkeys, &mut inspected, true, 0);
    }

    return inspected.values().map(|x| *x).sorted().rev().take(2).reduce(|prev, val| prev * val).unwrap();
}

fn second_star(monkeys: &Vec<Monkey>, common_denom: u128) -> u128 {
    let mut inspected: HashMap<usize, u128> = HashMap::new();

    for _i in 0..10000 {
        simulate_round(monkeys, &mut inspected, false, common_denom);
    }

    return inspected.values().map(|x| *x).sorted().rev().take(2).reduce(|prev, val| prev * val).unwrap();
}

pub fn solve() -> Result<(), io::Error> {
    let mut monkeys : Vec<Monkey> = Vec::new();
    let commands : Vec<String> = utils::get_lines("day_11").collect();
    let mut common_denom = 1;

    for chunk in commands.chunks(7) {
        let items : Vec<u128> = chunk[1]
            .replace("  Starting items: ", "")
            .split(", ")
            .map(|x| x.parse::<u128>().unwrap())
            .collect();
        
        let operation : String = chunk[2].replace("  Operation: new = ", "");

        let test_div = chunk[3]
            .replace("  Test: divisible by ", "").parse::<u128>().unwrap();
        common_denom *= test_div;

        let cond1 = chunk[4]
            .replace("    If true: throw to monkey ", "").parse::<usize>().unwrap();

        let cond2 = chunk[5]
            .replace("    If false: throw to monkey ", "").parse::<usize>().unwrap();
        
        let monko = Monkey::create(operation, test_div, (cond1,cond2), items);
        monkeys.push(monko);
    }

    println!("First Star: {:?}", first_star(&monkeys.clone()));
    println!("Second Star: {:?}", second_star(&monkeys, common_denom));

    return Ok(())
}