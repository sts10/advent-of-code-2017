use std::io::BufReader;
use std::io::BufRead;
// use std::path::Path;
use std::fs::File;
// use std::io::prelude::*;

fn main() {
    let mut sum: usize = 0;
    // read input from txt file 
    let file_name = "input.tsv";
    // let file_name = "test.tsv";
    let f = File::open(file_name).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l); 
        let str_vec: Vec<&str>= l.split("	").collect();
        // let this_vec: Vec<usize>= l.split("	").filter_map(|s| s.to_digit(10)).collect();

        let mut d_vec: Vec<usize> = [].to_vec();
        for s in str_vec {
            d_vec.push(s.parse::<usize>().unwrap());
        }
        // let data: Vec<u32> = l.spl.filter_map(|s| s.to_digit(10)).collect();
        // find min + max
        // add it to sum
        let v_max = d_vec.iter().max().unwrap();
        let v_min = d_vec.iter().min().unwrap();
        sum = v_max - v_min + sum;

    }          
    // let contents: Vec<u32> = file_to_vec(file_name);
    println!("sum is {}", sum);
}
