use std::env;
use std::fs;

// TODO run scenarios as tests instead of file input
fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Reading values from file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let mut most_calories = 0;
    let mut elf_calories = 0;
    contents.lines().for_each(|line| {
        elf_calories = if line.parse::<i32>().is_err() {
            if elf_calories > most_calories {
                most_calories = elf_calories;
            }
            0
        } else {
            let calories = line.parse::<i32>().unwrap(); 
            elf_calories + calories
        };
    });

    println!("Most calories: {}", most_calories)
}