use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];
    println!("query: {query}");
    println!("In file_name: {file_path}");

    let contents = fs::read_to_string(file_path)
    .expect("should have been able to read the file");
    println!("Content inside the file: \n{contents}");
}