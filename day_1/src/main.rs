use std::fs;

fn main() {
    println!("Advent of Code day 1!");

    let mut num_increased = 0;

    let input_file = fs::read_to_string("src/input.txt")
        .expect("Should have been able to read the file");

    let mut lineIterator = input_file.split(" ");

    for s in lineIterator {
        println!("{s}");
        let input_to_int: i32 = str().parse().unwrap();
        
    }
}
