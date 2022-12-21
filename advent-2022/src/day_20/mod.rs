use std::{ io, collections::HashMap };

use itertools::Itertools;

use crate::utils::{get_lines};

fn perform_mixing(original: &Vec<(i128,i128)>, current: &Vec<(i128,i128)>) -> Vec<(i128,i128)> {
    let arr_size = original.len() as i128;

    let mut nums_with_idx = current.clone();
    let mut current_pos_map : HashMap<(i128, i128), i128> = current.iter().enumerate().map(|(idx,&key)| (key, idx as i128)).collect();

    for &(original_pos, num) in original {
        if num == 0 { continue }
        let current_pos = *current_pos_map.get(&(original_pos, num)).unwrap();
        let mut _new_pos : i128 = 0;
        
        // Every arr_size - 1 times, we are on the same configuration
        let new_num = num % (arr_size - 1);

        _new_pos = current_pos + (new_num% arr_size) + (new_num/ arr_size);

        if _new_pos >= arr_size { 
            _new_pos = _new_pos - arr_size +1
        } 
        if _new_pos <= 0 { 
            _new_pos = arr_size + _new_pos - 1 
        }

        if current_pos != _new_pos {
            // now we update the nums
            nums_with_idx.remove(current_pos as usize);
            nums_with_idx.insert(_new_pos as usize, (original_pos, num));
    
            // and now we update the current pos map
            current_pos_map = nums_with_idx.iter()
                .enumerate()
                .map(|(new_idx, &key)| (key, new_idx as i128))
                .collect();
        }
    }

    return nums_with_idx;
}

fn decrypt(nums: &Vec<i128>, mixins: usize) -> i128 {
    let original = nums.iter().enumerate().map(|(idx,&x)| (idx as i128,x)).collect_vec();
    let current = original.clone();

    let mut current = perform_mixing(&original, &current);

    for _i in 1..mixins {
        current = perform_mixing(&original, &current);
    }
    
    let mut mixed_nums_iter = current.iter().cycle();

    // iterate until 0
    let mut val = -1;
    while val != 0 {
        val = mixed_nums_iter.by_ref().next().unwrap().1;
    }
    
    let n1000 = mixed_nums_iter.by_ref().skip(999).next().unwrap().1;
    let n2000 = mixed_nums_iter.by_ref().skip(999).next().unwrap().1;
    let n3000 = mixed_nums_iter.by_ref().skip(999).next().unwrap().1;

    return n1000 + n2000 + n3000;
}

pub fn solve() -> Result<(), io::Error> {
    let nums = get_lines("day_20").map(|x| x.parse::<i128>().unwrap()).collect_vec();
    println!("First Star: {:?} ", decrypt(&nums, 1));
    // RUNS IN 10 SECONDS
    let nums2 = nums.iter().map(|&x| x * 811589153).collect_vec();
    println!("Second Star: {:?} ", decrypt(&nums2, 10));

    return Ok(())
}