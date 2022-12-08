use std::{ io, cmp };
use itertools::Itertools;

use crate::utils::{self};

fn get_visible(trees : &Vec<Vec<i32>>) -> i32 {
    // get max until idx on rows. Adding a tuple with the first ocurrence idx
    let mut lr_row: Vec<Vec<(usize,i32)>> = trees.clone().into_iter()
        .map(|r| r.iter().enumerate().map(|c| (c.0,*c.1)).collect()).collect_vec();

    let mut rl_row: Vec<Vec<(usize,i32)>> = trees.clone().into_iter()
        .map(|r| r.iter().enumerate().map(|c| (c.0,*c.1)).collect()).collect_vec();

    for row_idx in 0..lr_row.len() {
        for el_idx in 1..lr_row[row_idx].len() {
            if lr_row[row_idx][el_idx-1].1 >= lr_row[row_idx][el_idx].1 {
                lr_row[row_idx][el_idx] = lr_row[row_idx][el_idx-1]
            }

            let rl_el_idx = lr_row.len() - el_idx;
            if rl_row[row_idx][rl_el_idx].1 >= rl_row[row_idx][rl_el_idx-1].1 {
                rl_row[row_idx][rl_el_idx-1] = rl_row[row_idx][rl_el_idx]
            }
        }
    }

    // get max until idx on cols. Adding a tuple with the first ocurrence idx
    let mut td_col: Vec<Vec<(usize,i32)>> = trees.clone().into_iter().enumerate()
    .map(|r| r.1.iter().map(|c| (r.0,*c)).collect()).collect_vec();

    let mut dt_col: Vec<Vec<(usize,i32)>> = trees.clone().into_iter().enumerate()
    .map(|r| r.1.iter().map(|c| (r.0,*c)).collect()).collect_vec();

    for col_idx in 0..td_col[0].len() {
        for el_idx in 1..td_col.len() {
            if td_col[el_idx-1][col_idx].1 >= td_col[el_idx][col_idx].1 {
                td_col[el_idx][col_idx] = td_col[el_idx-1][col_idx]
            }

            let dt_el_idx = lr_row.len() - el_idx;
            if dt_col[dt_el_idx][col_idx].1 >= dt_col[dt_el_idx-1][col_idx].1 {
                dt_col[dt_el_idx-1][col_idx] = dt_col[dt_el_idx][col_idx]
            }
        }
    }
    
    // Count visible
    let mut count_visible = 0;
    for row_idx in 0..trees.len() {
        for col_idx in 0..trees[row_idx].len() {
            if lr_row[row_idx][col_idx] == (col_idx, trees[row_idx][col_idx])
            || rl_row[row_idx][col_idx] == (col_idx, trees[row_idx][col_idx])
            || td_col[row_idx][col_idx] == (row_idx, trees[row_idx][col_idx])
            || dt_col[row_idx][col_idx] == (row_idx, trees[row_idx][col_idx]) {
                count_visible += 1
            }
        }
    }

    return count_visible;
}

fn get_max_score(trees : &Vec<Vec<i32>>) -> i32 {
    let mut max_score = 0;
    for row_idx in 0..trees.len() {
        for col_idx in 0..trees[row_idx].len() {
            let mut curr_score = 1;
            // Check lr
            let mut c = 0;
            for it in col_idx+1..trees[row_idx].len() {
                c+= 1;
                if trees[row_idx][it] >= trees[row_idx][col_idx] {
                    if it < trees[row_idx].len()-1 && trees[row_idx][it+1] <= trees[row_idx][it] { break }
                }
            }
            curr_score *= c;

            // Check rl
            let mut c = 0;
            for it in (0..col_idx).rev() {
                c+= 1;
                if trees[row_idx][it] >= trees[row_idx][col_idx] {
                    if it > 0 && trees[row_idx][it-1] <= trees[row_idx][it] { break }
                }
            }
            curr_score *= c;

            // Check tb
            let mut c = 0;
            for it in row_idx+1..trees.len() {
                c+= 1;
                if trees[it][col_idx] >= trees[row_idx][col_idx] {
                    if it < trees.len()-1 && trees[it+1][col_idx] <= trees[it][col_idx] { break }
                }
            }
            curr_score *= c;

            // Check bt
            let mut c = 0;
            for it in (0..row_idx).rev() {
                c+= 1;
                if trees[it][col_idx] >= trees[row_idx][col_idx] {
                    if it > 0 && trees[it-1][col_idx] <= trees[it][col_idx] { break }
                }
            }
            curr_score *= c;
            max_score = cmp::max(max_score, curr_score);
        }
    }

    return max_score;
}


pub fn solve() -> Result<(), io::Error>{
    let lines : Vec<String> = utils::get_lines("day_8").collect();
    let trees : Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as i32).collect_vec())
        .collect();

    println!("First Star: {:?}", get_visible(&trees));
    println!("Second Star: {:?}", get_max_score(&trees));
    return Ok(())
}