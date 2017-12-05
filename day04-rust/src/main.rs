
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let file_name = "input.txt";

    println!("Part 1 is {}", part_1(file_name));
    println!("Part 2 is {}", part_2(file_name));
}

fn part_1(filename: &str) -> (usize){
    let mut num_of_valid: usize = 0;
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l); 
        // let mut str_vec: Vec<&str>= l.split("	").collect();
        let mut str_vec: Vec<&str>= l.split(" ").collect();
        let original_length = str_vec.len();
        // sort vector alphabeticallly
        str_vec.sort();
        str_vec.dedup();
        let dedup_length = str_vec.len();
        if original_length == dedup_length{
            println!("valid!");
            num_of_valid = num_of_valid + 1;
        } else {
            println!("not valid!");
        }
    }          
    return num_of_valid;
}



fn part_2(filename: &str) -> (usize){
    let mut num_of_valid: usize = 0;
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l); 
        let mut str_vec: Vec<&str>= l.split(" ").collect();
        let original_length = str_vec.len();
        // sort vector alphabeticallly
        str_vec.sort();
        let mut sorted_str_vec: Vec<String> = [].to_vec();
        for s in str_vec {
            let mut sorted_s = sort_string(s);
            sorted_str_vec.push(sorted_s);
        }
        // sort again
        sorted_str_vec.sort();
        sorted_str_vec.dedup();
        let dedup_length = sorted_str_vec.len();
        if original_length == dedup_length{
            println!("valid!");
            num_of_valid = num_of_valid + 1;
        } else {
            println!("not valid!");
        }
    }          
    return num_of_valid;
}

fn sort_string(s: &str) -> String {
    let mut temp = s.chars().collect::<Vec<char>>();
    temp.sort();
    temp.into_iter().collect::<String>()
}
