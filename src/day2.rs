use std::{env, fs, path::PathBuf};

pub fn day2() {

    // read input
    let relative_input_path: &str = "input_files\\day2_input.txt";
    let mut path_to_input = PathBuf::from(env::current_dir().unwrap().as_path());
    path_to_input.push(relative_input_path);
    println!("{}", path_to_input.display());
    let content = fs::read_to_string(relative_input_path).unwrap();

    // process input
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;
    let mut sum_correct: u32 = 0;
    let mut sum_powers: u32 = 0;
    for line in content.split("\n") {
        let game = line.trim() .split(":").collect::<Vec<&str>>();
        let game_id = game[0].split(' ').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let draws = game[1].split(';');

        let mut highest_red: u32 = 0;
        let mut highest_green: u32 = 0;
        let mut highest_blue: u32 = 0;
        for draw in draws{
            let cubes = draw.split(',');
            for cube in cubes{
                let cube_string = cube.trim().split(' ').collect::<Vec<&str>>();
                let count = cube_string[0].trim().parse::<u32>().unwrap();
                let color = cube_string[1].trim();

                if color == "red" && count > highest_red{
                    highest_red = count;
                } else if color == "green" && count > highest_green{
                    highest_green = count;
                } else if color == "blue" && count > highest_blue{
                    highest_blue = count;
                }
            }
        }

        sum_powers += highest_red * highest_green * highest_blue;

        if max_red >= highest_red && max_green >= highest_green && max_blue >= highest_blue{
            sum_correct += game_id;
        }
        
    }

    println!("Sum of the correct game ids: {}", sum_correct);
    println!("Sum of the game powers: {}", sum_powers);
}

