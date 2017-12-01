use std::fs::File;
use std::io::prelude::*;

fn main() {
    // read input from txt file
    let mut f = File::open("input1.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let first_digit = contents.chars().nth(0).unwrap().to_digit(10).unwrap();
    let mut i: usize = 1;
    
    let mut sum: u32 = 0;
    let mut before_digit: u32 = 0;
    for c in contents.trim().chars() { 
        let this_digit = c.to_digit(10).unwrap();

        if i < contents.chars().count() && this_digit == before_digit{
            sum = sum + this_digit;
        }

        // if at the end onf the list, check against first digit
        if i == contents.chars().count() - 1 && this_digit == first_digit{
            sum = sum + this_digit;
        }
        before_digit = this_digit;
        i = i + 1;
    }
    println!("sum is {}", sum);
}
