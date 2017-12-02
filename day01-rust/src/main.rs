use std::fs::File;
use std::io::prelude::*;

fn main() {
    // read input from txt file
    let file_name = "input1.txt";
    let contents: Vec<u32> = file_to_vec(file_name);

    println!("part 1 sum is {}", part_1(&contents));
    println!("part 2 sum is {}", part_2(&contents));
}

fn part_1(contents: &Vec<u32>) -> (u32){
    let mut sum: u32 = 0;
    // initialize before_digit to the last digit of contents
    let mut before_digit: u32 = contents[contents.len()-1];

    for d in contents { 
        if d == &before_digit{
            sum = sum + d;
        }
        before_digit = *d;
    }
    return sum;
}

fn part_2(contents: &Vec<u32>) -> (u32){
    let mut sum: u32 = 0;
    let step: usize = contents.len() / 2;
    // println!("step is {}", step);

    let mut i: usize = 0;
    while i < step {
        if contents[i] == contents[i+step]{
            sum = sum + contents[i] + contents[i+step];
        }
        i = i + 1;
    }
    return sum;
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


// 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the
// third digit (2) matches the fourth digit.
#[test]
fn part_1_example1() {
    // let contents = vec![1,1,2,2];
    // assert_eq!(part_1(&contents), 3);
    assert_eq!(part_1(&vec![1,1,2,2]), 3);
}

// 1111 produces 4 because each digit (all 1) matches the next.
#[test]
fn part_1_example2() {
    assert_eq!(part_1(&vec![1,1,1,1]), 4);
}

// 1234 produces 0 because no digit matches the next.
#[test]
fn part_1_example3() {
    assert_eq!(part_1(&vec![1,2,3,4]), 0);
}

// 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
#[test]
fn part_1_example4() {
    assert_eq!(part_1(&vec![9,1,2,1,2,1,2,9]), 9);
}



// 1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
#[test]
fn part_2_example1() {
    assert_eq!(part_2(&vec![1,2,1,2]), 6);
}

// 1221 produces 0, because every comparison is between a 1 and a 2.
#[test]
fn part_2_example2() {
    assert_eq!(part_2(&vec![1,2,2,1]), 0);
}

// 123425 produces 4, because both 2s match each other, but no other digit has a match.
#[test]
fn part_2_example3() {
    assert_eq!(part_2(&vec![1,2,3,4,2,5]), 4);
}

// 123123 produces 12.
#[test]
fn part_2_example4() {
    assert_eq!(part_2(&vec![1,2,3,1,2,3]), 12);
}

// 12131415 produces 4
#[test]
fn part_2_example5() {
    assert_eq!(part_2(&vec![1,2,1,3,1,4,1,5]), 4);
}
