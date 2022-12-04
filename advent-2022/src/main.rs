mod day_1;
mod day_2;
mod day_3;
mod day_4;

pub mod utils;
fn main() {
    let problem = 4;

    match problem {
        1 => day_1::solve().unwrap(),
        2 => day_2::solve().unwrap(),
        3 => day_3::solve().unwrap(),
        4 => day_4::solve().unwrap(),

        _ => println!("Not solved yet!")
    }
}
