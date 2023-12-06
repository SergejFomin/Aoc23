use std::{env, fs, path::PathBuf};

pub fn day1(digits_only: bool) {
    // init string digits
    let mut string_digits: Vec<StringDigit> = vec![];
    if !digits_only {
        string_digits.push(StringDigit {string: "one",digit: 1,});
        string_digits.push(StringDigit {string: "two",digit: 2,});
        string_digits.push(StringDigit {string: "three",digit: 3,});
        string_digits.push(StringDigit {string: "four",digit: 4,});
        string_digits.push(StringDigit {string: "five",digit: 5,});
        string_digits.push(StringDigit {string: "six",digit: 6,});
        string_digits.push(StringDigit {string: "seven",digit: 7,});
        string_digits.push(StringDigit {string: "eight",digit: 8,});
        string_digits.push(StringDigit {string: "nine",digit: 9,});
    }

    // read input
    let day1_relative_input_path: &str = "input_files\\day1_input.txt";
    let mut path_to_day1_input = PathBuf::from(env::current_dir().unwrap().as_path());
    path_to_day1_input.push(day1_relative_input_path);
    println!("{}", path_to_day1_input.display());
    let content = fs::read_to_string(day1_relative_input_path).unwrap();

    // porcess
    let mut sum: u32 = 0;
    for line in content.split("\n") {
        if digits_only {
            sum += get_numbers_1(line.to_string());
        } else {
            sum += get_numbers_2(line.to_string(), &string_digits);
        }
    }

    println!("{}", sum);
}

fn get_numbers_1(string: String) -> u32 {
    let radix = 10;
    let mut number: u32 = 0;

    for character in string.chars() {
        if character.is_numeric() {
            let digit: u32 = character.to_digit(radix).unwrap();
            number += digit * 10;
            break;
        }
    }

    for character in string.chars().rev() {
        if character.is_numeric() {
            let digit: u32 = character.to_digit(radix).unwrap();
            number += digit;
            break;
        }
    }

    return number;
}

fn get_numbers_2(input: String, possible_string_digits: &Vec<StringDigit>) -> u32 {
    let radix = 10;

    // get the first digit
    // by digit char
    let mut first_digit_index: usize = 0;
    let mut first_digit: u32 = 0;
    let mut counter: usize = 0;
    for character in input.chars() {
        if character.is_numeric() {
            first_digit = character.to_digit(radix).unwrap();
            first_digit_index = counter;
            break;
        }

        counter += 1;
    }

    // by string
    for index in 0usize..=input.len() -1{
        let substring : String = input.chars().skip(index).collect();
        let digit: u32 = find_starting_string_digit(substring, possible_string_digits);
        if digit > 0 && (first_digit == 0 || first_digit_index > index){
            first_digit = digit;
            break;
        }
    }

    // get the seconds digit
    // by digit char
    let mut second_digit_index: usize = 0;
    let mut second_digit: u32 = 0;
    counter = input.len() - 1;
    for character in input.chars().rev() {
        if character.is_numeric() {
            second_digit = character.to_digit(radix).unwrap();
            second_digit_index = counter;
            break;
        }

        if counter > 0{
            counter -= 1;
        }
    }

    // by string
    for index in (0usize..=input.len() -1).rev(){
        let substring : String = input.clone().chars().skip(index).collect();
        let digit: u32 = find_starting_string_digit(substring, possible_string_digits);
        if digit > 0 && (second_digit == 0 || index > second_digit_index ){
            second_digit = digit;
            break;
        }
    }

    return (first_digit * 10) + second_digit;
}

fn find_starting_string_digit(string: String, possible_string_digits: &Vec<StringDigit>) -> u32 {
    for possible_string_digit in possible_string_digits{
        if string.starts_with(possible_string_digit.string){
            return possible_string_digit.digit;
        }
    }

    return 0;
}

struct StringDigit<'a> {
    string: &'a str,
    digit: u32,
}