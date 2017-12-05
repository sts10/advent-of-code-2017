use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let file_name = "input.txt";

    println!("Part 1 is {}", part_1(file_name));
    println!("Part 2 is {}", part_2(file_name));
}

fn part_1(filename: &str) -> (usize){
    let mut index: usize = 0;
    let mut list: Vec<i64> = [].to_vec();
    let mut num_of_steps: usize = 0;
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        // println!("{}", l); 

        // convert l to i64
        let n: i64 = l.parse::<i64>().unwrap();
        println!("n is {}", n); 
        // add l as i64 to a vector called list
        list.push(n);
    }          
    while index < list.len() {
        let this_jump: i64 = list[index];
        let new_index: usize;
        if this_jump >= 0 {
            new_index = index + this_jump.abs() as usize;
        } else {
            new_index = index - this_jump.abs() as usize;
        }
        // increment by 1
        list[index] = list[index] + 1;
        index = new_index;

        num_of_steps = num_of_steps + 1;
    }
    return num_of_steps;
}



fn part_2(filename: &str) -> (usize){
    let mut index: usize = 0;
    let mut list: Vec<i64> = [].to_vec();
    let mut num_of_steps: usize = 0;
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        // println!("{}", l); 

        // convert l to i64
        let n: i64 = l.parse::<i64>().unwrap();
        println!("n is {}", n); 
        // add l as i64 to a vector called list
        list.push(n);
    }          
    while index < list.len() {
        let this_jump: i64 = list[index];
        let new_index: usize;
        if this_jump >= 0 {
            new_index = index + this_jump.abs() as usize;
        } else {
            new_index = index - this_jump.abs() as usize;
        }

        if this_jump >= 3 {
            list[index] = list[index] - 1;
        } else {
            // increment by 1
            list[index] = list[index] + 1;
        }

        index = new_index;
        num_of_steps = num_of_steps + 1;
    }
    return num_of_steps;
}

