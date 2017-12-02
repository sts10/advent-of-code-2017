use std::io::BufReader;
use std::io::BufRead;
// use std::path::Path;
use std::fs::File;
// use std::io::prelude::*;

fn main() {
    let file_name = "input.tsv";

    println!("Part 1 is {}", part_1(file_name));
    println!("Part 2 is {}", part_2("input.tsv"));
}

// my part 1 correct answer is 21845
fn part_1(filename: &str) -> (usize){
    let mut sum: usize = 0;
    let f = File::open(filename).unwrap();
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
    return sum;
}
// 133 is too low for my input
// 191 is my correct answer
fn part_2(filename: &str) -> (usize){
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let mut sum: usize = 0;
    let mut d_vec: Vec<usize> = [].to_vec();

    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l); 
        let str_vec: Vec<&str>= l.split("	").collect();
        // convert to vector of digits
        for s in str_vec {
            d_vec.push(s.parse::<usize>().unwrap());
        }
        let mut r: usize = 0;
        while d_vec.len() > 1{
            r = try_last(&mut d_vec);
            println!("r is {}", r);
            if r > 0{
                break;
            }
        }
        sum = sum + r;
        // reset d_vec
        d_vec = [].to_vec();
    }
    return sum;
}

fn try_last(d_vec: &mut Vec<usize>) -> usize{
    let n: usize = d_vec.pop().unwrap();
    println!("n is {}", n);
    println!("lenght of d_vec is {}", d_vec.len());
    for e in d_vec {
        if n < *e && (*e % n == 0){
            return (*e / n) as usize;
        } else if n % *e == 0{
            return (n / *e) as usize;
        } 
    }
    return 0;
}


#[test]
fn part_1_test() {
    assert_eq!(part_1("input.tsv"), 21845);
}

// Part 2 tests
#[test]
fn part_2_example() {
    assert_eq!(part_2("test2.tsv"), 9);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2("input.tsv"), 191);
}
