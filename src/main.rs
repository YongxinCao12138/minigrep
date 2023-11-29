use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let search_string = &args[1];
    let file_path = &args[2];

    println!("search string:{search_string}");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
