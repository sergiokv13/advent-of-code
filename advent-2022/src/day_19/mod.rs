use std::{ io };

use itertools::Itertools;

use crate::utils::{get_lines};

#[derive(Debug, Clone, Copy)]
struct Stock {
    ore: i128,
    clay: i128,
    obsidian: i128,
    geode: i128,
}

impl Stock {
    pub fn add(&mut self, s : Stock) {
        self.ore += s.ore;
        self.clay += s.clay;
        self.obsidian += s.obsidian;
        self.geode += s.geode;
    }
}

#[derive(Debug, Clone, Copy)]
struct BluePrint {
    ore_for_ore: i128,
    ore_for_clay: i128,
    ore_for_obsidian: i128,
    clay_for_obsidian: i128,
    ore_for_geode: i128,
    obsidian_for_geode: i128,
}

impl BluePrint {
    pub fn create(ore: i128, clay: i128, obsidian: (i128,i128), geode: (i128,i128)) -> BluePrint {
        let b = BluePrint {
            ore_for_ore: ore,
            ore_for_clay: clay,
            ore_for_obsidian: obsidian.0,
            clay_for_obsidian: obsidian.1,
            ore_for_geode: geode.0,
            obsidian_for_geode: geode.1,
        };
        return b;
    }
    pub fn get_possible_robots(self, robots: Stock, materials: Stock) -> (Vec<(Stock, Stock)>, bool) {
        let mut possible : Vec<(Stock, Stock)> = Vec::new();

        if materials.ore >= self.ore_for_geode &&  materials.obsidian >= self.obsidian_for_geode {
            possible.push((
                Stock { ore: robots.ore, clay: robots.clay, obsidian: robots.obsidian, geode: robots.geode + 1 },
                Stock { ore: materials.ore - self.ore_for_geode, clay: materials.clay, obsidian: materials.obsidian - self.obsidian_for_geode, geode: materials.geode }
            ));
            // If we can make a geode robot, that is the only option
            return (possible, true);
        }

        if materials.ore >= self.ore_for_obsidian &&  materials.clay >= self.clay_for_obsidian {
            possible.push((
                Stock { ore: robots.ore, clay: robots.clay, obsidian: robots.obsidian + 1, geode: robots.geode },
                Stock { ore: materials.ore - self.ore_for_obsidian, clay: materials.clay - self.clay_for_obsidian, obsidian: materials.obsidian, geode: materials.geode }
            ));
            // if we can make an obsidian robot if can't make a geode, that is the only option
            return (possible, true);
        }

        if materials.ore >= self.ore_for_ore {
            possible.push((
                Stock { ore: robots.ore + 1, clay: robots.clay, obsidian: robots.obsidian, geode: robots.geode },
                Stock { ore: materials.ore - self.ore_for_ore, clay: materials.clay, obsidian: materials.obsidian, geode: materials.geode }
            ));
        }

        if materials.ore >= self.ore_for_clay {
            possible.push((
                Stock { ore: robots.ore, clay: robots.clay + 1, obsidian: robots.obsidian, geode: robots.geode },
                Stock { ore: materials.ore - self.ore_for_clay, clay: materials.clay, obsidian: materials.obsidian, geode: materials.geode }
            ));
        }

        return (possible, false);
    }
}

fn get_max_geodes ( 
    blueprint: &BluePrint, 
    robots: Stock, 
    mut materials: Stock, 
    minutes: i128,
    max_geodes: &mut [i128],
) {    
    max_geodes[0] = max_geodes[0].max(materials.geode);
    if minutes == 0 { return; }

    // let's assume we can create a geode robot every remaining minute
    // materials.geode + robots.geode + (robots.geode + 1) + (robots.geode + 2) + (robots.geode + 3) + ...
    // materials.geode + minutes(robots.geode) + ( 1 + 2 + 3 + 4 + ... + minutes)
    // materials.geode + minutes(robots.geode) + (minutes * (minutes + 1) / 2) -> max number of geodes we can build

    if materials.geode + minutes * (robots.geode) + (minutes * (minutes + 1) / 2) < max_geodes[0] {
        // this won't lead us anywhere
        return;
    }

    let (opts, short_circuited) = blueprint.get_possible_robots(robots.clone(), materials.clone());
    for (new_robots, mut new_materials) in opts {
        new_materials.add(robots);
        get_max_geodes(
            blueprint, 
            new_robots, 
            new_materials, 
            minutes - 1,
            max_geodes,
        )
    }

    if !short_circuited {
        // If we don't create any robot
        // if we created a geode or obsidian robot, there is no sense in keep going
        materials.add(robots);
        get_max_geodes(
            blueprint, 
            robots, 
            materials, 
            minutes - 1,
            max_geodes,
        );
    }
}

// RUNS IN 5 min aprox
fn get_first_star ( blueprints: Vec<BluePrint> ) -> i128 {
    let mut sol = 0;
    for (idx, blueprint) in blueprints.into_iter().enumerate() {
        let mut max_geodes = [0];

        let robots = Stock { ore: 1, clay: 0, obsidian: 0, geode: 0 };
        let materials = Stock { ore: 0, clay: 0, obsidian: 0, geode: 0 };

        get_max_geodes(&blueprint, robots, materials, 24, &mut max_geodes);
        sol += max_geodes[0] * (idx as i128 + 1);
    
        println!("Max for blueprint {:?} : {:?}", idx, max_geodes);
    }
    return sol;
}

// RUNS IN 5 min aprox
fn get_second_star ( blueprints: Vec<BluePrint> ) -> i128 {
    let mut sol = 1;
    for blueprint in blueprints.into_iter().take(3) {
        let mut max_geodes = [0];

        let robots = Stock { ore: 1, clay: 0, obsidian: 0, geode: 0 };
        let materials = Stock { ore: 0, clay: 0, obsidian: 0, geode: 0 };

        get_max_geodes(&blueprint, robots, materials, 32, &mut max_geodes);
        sol *= max_geodes[0];
    
        println!("Max for blueprint : {:?}", max_geodes);
    }
    return sol;
}

pub fn solve() -> Result<(), io::Error> {
    let blueprints = get_lines("day_19").map(|x| {
        let splitted = x.split(" costs ").collect_vec();
        let ore = splitted[1].split_whitespace().next().unwrap().parse::<i128>().unwrap();
        let clay = splitted[2].split_whitespace().next().unwrap().parse::<i128>().unwrap();

        let (obs1, obs2) = splitted[3]
            .replace(" ore and ", " ")
            .replace(" clay. Each geode robot","")
            .split_whitespace().map(|n| n.parse::<i128>().unwrap())
            .collect_tuple()
            .unwrap();

        let (geo1, geo2) = splitted[4]
            .replace(" ore and ", " ")
            .replace(" obsidian.","")
            .split_whitespace().map(|n| n.parse::<i128>().unwrap())
            .collect_tuple()
            .unwrap();

        return BluePrint::create(ore, clay, (obs1,obs2), (geo1,geo2))
    }).collect_vec();
    
    println!("First Star: {:?} ", get_first_star(blueprints.clone()));
    println!("Second Star: {:?} ", get_second_star(blueprints));

    return Ok(())
}