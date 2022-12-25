use std::{ io };
use itertools::Itertools;

use crate::utils::{get_lines};

fn snafu_to_number(snafu : Vec<char>) -> i128 {
    let mut exp = 0;
    let mut res = 0;
    for i in (0..snafu.len()).rev() {
        match snafu[i] {
            '-' => { res -= (5 as i128).pow(exp); },
            '=' => { res -= (5 as i128).pow(exp) * 2; },
            _ => {  res += snafu[i].to_digit(10).unwrap() as i128 * (5 as i128).pow(exp); }
        }
        exp += 1;
    }

    return res;
}

fn number_to_snafu(number: i128) -> String {
    if number == 0 { return "".to_string() }

    let remaining = (number + 2) % 5;
    let div = (number + 2) / 5;

    return number_to_snafu(div) + match remaining {
        0 => "=",
        1 => "-",
        2 => "0",
        3 => "1",
        4 => "2",
        _ => unreachable!(),
    }
}

pub fn solve() -> Result<(), io::Error> {
    let snafu_numbers = get_lines("day_24")
        .map(|line| line.chars().collect_vec());
    let snafu_sum : i128 = snafu_numbers.map(|snafu| snafu_to_number(snafu)).sum();
    println!("First Star: {:?}", number_to_snafu(snafu_sum));

    return Ok(())
}