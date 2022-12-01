mod day_1;

pub mod utils;
fn main() {
    let problem = 1;

    match problem {
        1 => day_1::solve().unwrap(),

        _ => println!("Not solved yet!")
    }
}
