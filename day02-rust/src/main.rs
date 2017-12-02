use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let file_name = "input.tsv";

    println!("Part 1 is {}", part_1(file_name));
    println!("Part 2 is {}", part_2(file_name));
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
            // try the last number of each line to see if it gives us a whole number division
            // result
            // if it does, it'll return it, here as r
            r = try_last(&mut d_vec);
            // since we know there will only be one whole number division per line, we can break
            // out of this while loop
            if r > 0{
                break;
            }
        }
        sum = sum + r;
        // reset d_vec for our next line!
        d_vec = [].to_vec();
    }
    return sum;
}

fn try_last(d_vec: &mut Vec<usize>) -> usize{
    // pop the last usize off the vector and try it against the rest of the vector
    let n: usize = d_vec.pop().unwrap();
    for e in d_vec {
        if n < *e && (*e % n == 0){
            return *e / n;
        } else if n % *e == 0{
            return n / *e;
        } 
    }
    // no whole number division found for this particular vector element
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
