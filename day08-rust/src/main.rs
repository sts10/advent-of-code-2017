use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
fn main() {
    // b inc 5 if a > 1
    // a inc 1 if b < 5
    // c dec -10 if a >= 1
    // c inc -20 if c == 10
    // let input_vec: Vec<String> = read_by_line("test-input.txt").unwrap();
    let input_vec: Vec<String> = read_by_line("input.txt").unwrap();

    let mut registers_hashmap: HashMap<&str, isize> = HashMap::new();

    let mut highest_count_at_any_one_time: isize = 0;

    // first, we'll seed registers_hashmap with each key from the instructions, and set the value
    // to 0
    for instructions in &input_vec {
        let split_instructions = instructions.split(" ");
        let instructions_vec: Vec<&str> = split_instructions.collect();
        registers_hashmap.entry(instructions_vec[0]).or_insert(0);
    }

    // Now we'll actually read each instruction
    for instructions in &input_vec {
        let split_instructions = instructions.split(" ");
        let instructions_vec: Vec<&str> = split_instructions.collect();
        println!("{:?}", instructions_vec);
        let to_compare_to: isize = instructions_vec[6].parse::<isize>().unwrap();

        println!("got to the if statements");
        // match instructions_vec[5] {
        if instructions_vec[5] == ">" {
            println!(
                "Right here, the left side of the comparison is {}",
                instructions_vec[4]
            );
            if registers_hashmap.get(&instructions_vec[4]) > Some(&to_compare_to) {
                println!("OK We're going to make this change");
                // if we're here we're going to making a change
                if instructions_vec[1] == "inc" {
                    println!("got to the and_modify");
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x += &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                } else if instructions_vec[1] == "dec" {
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x -= &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                }
            }
        } else if instructions_vec[5] == "<" {
            if registers_hashmap.get(&instructions_vec[4]) < Some(&to_compare_to) {
                // if we're here we're going to making a change
                if instructions_vec[1] == "inc" {
                    println!(
                        "BEFORE the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x += &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                    println!(
                        "AFTER the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                } else if instructions_vec[1] == "dec" {
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x -= &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                }
            }
        } else if instructions_vec[5] == ">=" {
            if registers_hashmap.get(&instructions_vec[4]) >= Some(&to_compare_to) {
                // if we're here we're going to making a change
                if instructions_vec[1] == "inc" {
                    println!(
                        "BEFORE the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x += &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                    println!(
                        "AFTER the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                } else if instructions_vec[1] == "dec" {
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x -= &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                }
            }
        } else if instructions_vec[5] == "<=" {
            if registers_hashmap.get(&instructions_vec[4]) <= Some(&to_compare_to) {
                // if we're here we're going to making a change
                if instructions_vec[1] == "inc" {
                    println!(
                        "BEFORE the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x += &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                    println!(
                        "AFTER the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                } else if instructions_vec[1] == "dec" {
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x -= &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                }
            }
        } else if instructions_vec[5] == "==" {
            if registers_hashmap.get(&instructions_vec[4]) == Some(&to_compare_to) {
                // if we're here we're going to making a change
                if instructions_vec[1] == "inc" {
                    println!(
                        "BEFORE the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x += &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                    println!(
                        "AFTER the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                } else if instructions_vec[1] == "dec" {
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x -= &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                }
            }
        } else if instructions_vec[5] == "!=" {
            if registers_hashmap.get(&instructions_vec[4]) != Some(&to_compare_to) {
                // if we're here we're going to making a change
                if instructions_vec[1] == "inc" {
                    println!(
                        "BEFORE the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x += &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                    println!(
                        "AFTER the entry call, registers_hashmap is {:?}",
                        registers_hashmap
                    );
                } else if instructions_vec[1] == "dec" {
                    registers_hashmap
                        .entry(instructions_vec[0])
                        .and_modify(|x| *x -= &instructions_vec[2].parse().unwrap())
                        .or_insert(0);
                }
            }
        }

        let mut count_vec: Vec<(&&str, &isize)> = registers_hashmap.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        if highest_count_at_any_one_time < *count_vec[0].1 {
            highest_count_at_any_one_time = *count_vec[0].1;
        }
    }

    // for part 1, sort by value
    //

    let mut count_vec: Vec<(&&str, &isize)> = registers_hashmap.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    for pair in &count_vec {
        println!("{:?}", pair);
    }

    let part_1_ans = count_vec[0].1;
    println!("answer to part 1 is {} (should be 5125)", part_1_ans);
    println!("answer to part 2 is {} ", highest_count_at_any_one_time);
}

fn read_by_line<T: FromStr>(file_path: &str) -> io::Result<Vec<T>> {
    let mut vec = Vec::new();
    let f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(_e) => {
                eprintln!("Error");
                continue;
            }
        }
    }
    Ok(vec)
}
