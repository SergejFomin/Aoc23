mod day1;
mod day2;

fn main() {
    let day = 2;
    match day{
        1 => day1::day1(false),
        2 => day2::day2(),
        _=>println!("No valid day selected ")
    }
}
