use std::fs::File;
use std::io::prelude::*;

fn main() {
    // read input from txt file
    let file_name = "input1.txt";
    let contents: Vec<u32> = file_to_vec(file_name);

    let mut i: usize = 1;
    let mut sum: u32 = 0;
    // initialize before_digit to the last digit of contents
    let contents_size = contents.len();
    let mut before_digit: u32 = contents[contents_size-1];

    for d in contents { 
        if i < contents_size && d == before_digit{
            sum = sum + d;
        }

        before_digit = d;
        i = i + 1;
    }
    println!("sum is {}", sum);
}

fn file_to_vec(filename: &str) -> (Vec<u32>){
    // read data_str from txt file
    let mut f = File::open(filename).expect("file not found");
    let mut data_str = String::new();
    f.read_to_string(&mut data_str)
        .expect("something went wrong reading the file");

    // convert it to a Vec
    let data: Vec<u32> = data_str.chars().filter_map(|s| s.to_digit(10)).collect();

    return data;
}
