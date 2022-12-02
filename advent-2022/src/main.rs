mod day_1;
mod day_2;

pub mod utils;
fn main() {
    let problem = 2;

    match problem {
        1 => day_1::solve().unwrap(),
        2 => day_2::solve().unwrap(),

        _ => println!("Not solved yet!")
    }
}
