use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    // Print a prompt
    println!("Specify file name:");

    // Create a mutable string to store the input
    let mut input = String::new();

    // Read input from the user
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let file = File::open(input.trim())
        .expect("Failed to open file");

    let reader = io::BufReader::new(file);

    let mut animal_vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        println!("  {}", line);
        animal_vec.push(line);
    }
    println!();

    let random_index = rand::random::<usize>() % animal_vec.len();
    let random_animal = &mut animal_vec[random_index];

    print_random_animal(random_animal);

    println!("Random animal: {}\n", random_animal);

    change_animal_name(random_animal);

    println!("Printing animals...\n");

    for animal in animal_vec.iter(){
        println!("  {}", animal);
    }
}

fn print_random_animal(animal_name: &String){
    println!("Hello, {}\n", animal_name);
}

fn change_animal_name(animal_name: &mut String){
    *animal_name = "==ANIMALS IDENTITY ERASED==".to_string();
}
