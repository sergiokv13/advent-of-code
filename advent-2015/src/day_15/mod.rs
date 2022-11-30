use std::{ io, collections::{HashMap} };
use itertools::Itertools;

use crate::utils::{self};

#[derive(Debug,Clone)]
struct Ingredient {
    capacity: i128,
    durability: i128,
    flavor: i128,
    texture: i128,
    calories: i128,
}

fn get_ingredient_by_spoons(ing_specs : &Ingredient, spoons: i128) -> Ingredient {
    Ingredient {
        capacity: ing_specs.capacity * spoons,
        durability: ing_specs.durability * spoons,
        flavor: ing_specs.flavor * spoons,
        texture: ing_specs.texture * spoons,
        calories: ing_specs.calories * spoons,
    }
}

fn merge_ingredients(ing1: Ingredient, ing2: Ingredient) -> Ingredient {
    let mut local_ing = ing1.clone();
    local_ing.capacity += ing2.capacity;
    local_ing.durability += ing2.durability;
    local_ing.flavor += ing2.flavor;
    local_ing.texture += ing2.texture;
    local_ing.calories += ing2.calories;
    return local_ing;
}

fn get_current_score(ing : &Ingredient) -> i128 {
    ing.capacity.max(0) * ing.durability.max(0) * ing.flavor.max(0) * ing.texture.max(0)
}

fn get_total_score_rec(
    ingredients: &HashMap<String, Ingredient>, 
    remaining_ing: &mut Vec<String>,
    remaining_spoons: i128,
) -> Vec<Ingredient> {
    let local_ing_name = &remaining_ing.clone()[remaining_ing.len()-1];
    let local_ing = ingredients.get(local_ing_name).unwrap();

    if remaining_ing.len() == 1 {
        return Vec::from([get_ingredient_by_spoons(local_ing, remaining_spoons)]);
    }

    remaining_ing.remove(remaining_ing.len()-1);
    let mut new_ings : Vec<Ingredient> = Vec::from([]);
    for i in 0..remaining_spoons+1 {
        let child_ings = get_total_score_rec(
            ingredients, 
            remaining_ing, 
            remaining_spoons - i,
        );
    
        let local_ing = get_ingredient_by_spoons(local_ing, i);
        for child in child_ings {
            new_ings.push(
                merge_ingredients(local_ing.clone(), child)
            )
        }
    };

    remaining_ing.push(local_ing_name.to_string());

    return new_ings;
}

fn get_total_score(
    ingredients: &HashMap<String, Ingredient>, 
    remaining_ing: &mut Vec<String>,
    remaining_spoons: i128,
) -> (i128,i128) {
    let mut by_perm : Vec<i128> = Vec::from([]);
    let mut by_perm2 : Vec<i128> = Vec::from([]);

    for perm in remaining_ing.iter().permutations(remaining_ing.len()) {
        let mut new_rem_ing : Vec<String> = perm.iter().map(|x| x.to_string()).collect();
    
        let ings = get_total_score_rec(
            ingredients, 
            &mut new_rem_ing, 
            remaining_spoons
        );

        by_perm.push(
            ings.iter().map(|i| get_current_score(&i)).max().unwrap()
        );

        by_perm2.push(
            ings.iter().filter(|x| x.calories == 500).map(|i| get_current_score(&i)).max().unwrap()
        );
    }
    
    return (
        *by_perm.iter().max().unwrap(),
        *by_perm2.iter().max().unwrap(),
    )
}

pub fn solve() -> Result<(), io::Error> {
    let lines = utils::get_lines("day_15");
    let mut ingredients : HashMap<String, Ingredient> = HashMap::new();

    for line in lines { 
        let clean_line = line
            .replace(": capacity", "")
            .replace(", durability", "")
            .replace(", flavor", "")
            .replace(", texture", "")
            .replace(", calories", "");

        let v_line : Vec<&str> = clean_line.split(" ").collect();
        let ing : Ingredient = Ingredient {
            capacity: v_line[1].parse::<i128>().unwrap(),
            durability: v_line[2].parse::<i128>().unwrap(),
            flavor: v_line[3].parse::<i128>().unwrap(),
            texture: v_line[4].parse::<i128>().unwrap(),
            calories: v_line[5].parse::<i128>().unwrap(),
        };
    
        ingredients.insert(v_line[0].to_string(), ing);
    }
    
    let mut remaining_ing : Vec<String> = ingredients.keys().map(|x| x.to_string()).collect();
    let res = get_total_score(&ingredients, &mut remaining_ing, 100);
    
    println!("First Star: {:?}", res.0);
    println!("Second Star: {:?}", res.1);
    
    return Ok(())
}