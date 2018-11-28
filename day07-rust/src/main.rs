use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let file_name = "input.txt";

    part_1(file_name);
    // println!("Part 1 is {}", part_1(file_name));
    // println!("Part 2 is {}", part_2(file_name));
}


fn part_1(filename: &str) {
    let mut being_held: Vec<String> = [].to_vec();
    let mut all_bases: Vec<String> = [].to_vec();

    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        // original code
        // let l = "fwft (72) -> ktlj, cntj, xhth";
        let str_vec: Vec<&str> = l.split(" ").collect();

        if str_vec.len() > 3 {
            for word in str_vec[3..str_vec.len()].iter() {
                being_held.push(word.replace(",", ""));
            }
            println!("str vec is {:?}", str_vec);
        }
        let base = str_vec[0];
        all_bases.push(String::from(str_vec[0]));
    }

    // now need to the word that is NOT being held
    all_bases.sort();
    being_held.sort();
    let mut index: usize = 0;
    for base in all_bases {
        if base != being_held[index]{
            println!("base is {}", base);
            println!("being held is {}", being_held[index]);
            println!("which means {} is the base that's never held", base);
            break;
        }
        index = index + 1;
    }

}
