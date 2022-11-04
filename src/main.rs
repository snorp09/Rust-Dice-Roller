use rand::prelude::*;
use std::string::String;
use clap::Parser;

#[derive(Parser)]
#[command(author, about)]
struct Args {

    ///How many sides on the die
    #[arg(short, long, default_value_t = 20)]
    dice_sides: i32,

    ///How many rolls
    #[arg(short, long, default_value_t = 1)]
    roll_count: i32,

    ///How many times we repeat the rolls
    #[arg(short, long, default_value_t = 1)]
    times_count: i32
}

fn roll_random(max_numb: &i32) -> i32{
    let mut rng = thread_rng();
    return rng.gen_range(1..=*max_numb);
}

fn main() {
    let args: Args = Args::parse();

    let dice_sides = args.dice_sides;
    let roll_count = args.roll_count - 1;
    let times_count = args.times_count - 1;

    for _count in 0..=times_count{
        let mut numbs = String::new();
        for count in 0..=roll_count {
            if count == roll_count{
                numbs.push_str(&*format!("{}", roll_random(&dice_sides).to_string()));
            } else {
                numbs.push_str(&*format!("{}, ", roll_random(&dice_sides).to_string()));
            }
        }
        println!("Results are: {}", numbs.trim())
    }


}
