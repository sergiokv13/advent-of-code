mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

pub mod utils;
fn main() {
    let problem = 17;

    use std::time::Instant;
    let now = Instant::now();

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
        10 => day_10::solve().unwrap(),
        11 => day_11::solve().unwrap(),
        12 => day_12::solve().unwrap(),
        13 => day_13::solve().unwrap(),
        14 => day_14::solve().unwrap(),
        15 => day_15::solve().unwrap(),
        16 => day_16::solve().unwrap(),
        17 => day_17::solve().unwrap(),

        _ => println!("Not solved yet!")
    }

    let elapsed = now.elapsed();
    println!("Running time: {:.2?}", elapsed);
}
