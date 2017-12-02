use std::io::BufReader;
use std::io::BufRead;
// use std::path::Path;
use std::fs::File;
// use std::io::prelude::*;

fn main() {
    let mut sum: usize = 0;
    let file_name = "input.tsv";
    let f = File::open(file_name).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l); 
        let str_vec: Vec<&str>= l.split("	").collect();
        // convert to vector of digits
        let mut d_vec: Vec<usize> = [].to_vec();
        for s in str_vec {
            d_vec.push(s.parse::<usize>().unwrap());
        }

        let v_max = d_vec.iter().max().unwrap();
        let v_min = d_vec.iter().min().unwrap();
        sum = v_max - v_min + sum;
    }          
    println!("sum is {}", sum);
}

// my part 1 correct answer is 21845
