
use std::{fs, env, path::PathBuf};

pub fn day1() {
    let day1_relative_input_path: &str = "input_files\\day1_input.txt";
    let mut path_to_day1_input = PathBuf::from(env::current_dir().unwrap().as_path());
    path_to_day1_input.push(day1_relative_input_path);
    println!("{}", path_to_day1_input.display());

    // read input
    let content = fs::read_to_string(day1_relative_input_path).unwrap();

    // porcess
    let mut sum: u32 = 0;
    for line in content.split("\n") {
        sum += get_numbers(line.to_string());
    }
    
    println!("{}", sum);
}

fn get_numbers(string: String) -> u32{
    let radix = 10;
    let mut number: u32 = 0;

    for character in string.chars(){
        if character.is_numeric(){
            let digit: u32 = character.to_digit(radix).unwrap();
            number += digit * 10;
            break;
        }
    }

    for character in string.chars().rev(){
        if character.is_numeric(){
            let digit: u32 = character.to_digit(radix).unwrap();
            number += digit;
            break;
        }
    }

    return number;
}