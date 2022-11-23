use std::{ io };

use crate::utils;

fn get_wrapping_paper(l:i128, w:i128, h:i128) -> (i128,i128) {
    let f1 = 2*l*w;
    let f2 = 2*w*h;
    let f3  = 2*h*l;

    let elems = [f1 / 2, f2 / 2, f3 / 2];
    let min = elems.iter().min().unwrap();

    let wrapping_paper = f1 + f2 + f3 + min;
    let volume = l * w * h;

    let mut sorted_dim = [l,w,h];
    sorted_dim.sort();
    let smallest_perimeter = sorted_dim[0] * 2 + sorted_dim[1] * 2;
    let rubber = smallest_perimeter + volume;

    return (wrapping_paper, rubber);
}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_2");
    let mut paper : i128 = 0;    
    let mut rubber : i128 = 0;

    for line in lines {
        let dim : Vec<i128> = line.split("x").map(|n| n.parse().unwrap()).collect();
        let res = get_wrapping_paper(dim[0], dim[1], dim[2]);
        paper = paper + res.0;
        rubber = rubber + res.1;
    }

    println!("First Star: {}", paper);
    println!("Second Star: {}", rubber);

    return Ok(())
}