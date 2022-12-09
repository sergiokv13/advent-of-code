mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

pub mod utils;
fn main() {
    let problem = 9;

    match problem {
        1 => day_1::solve().unwrap(),
        2 => day_2::solve().unwrap(),
        3 => day_3::solve().unwrap(),
        4 => day_4::solve().unwrap(),
        5 => day_5::solve().unwrap(),
        6 => day_6::solve().unwrap(),
        7 => day_7::solve().unwrap(),
        8 => day_8::solve().unwrap(),
        9 => day_9::solve().unwrap(),


        _ => println!("Not solved yet!")
    }
}
