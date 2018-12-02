use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
fn main() {
    let file_name = "input.txt";
    let river: Vec<char> = read_string_from_file_to_vector(file_name).unwrap();

    let mut number_of_nests = 0;
    let mut score = 0;
    let mut garbage_count = 0;
    let mut skip_next_char = false;
    let mut stop_reading = false;

    for c in river {
        if !skip_next_char && c == '!' {
            skip_next_char = true;
            continue;
        }
        if !(skip_next_char || (stop_reading && c != '>')) {
            match c {
                '{' => {
                    number_of_nests += 1;
                }
                '}' => {
                    score += number_of_nests;
                    number_of_nests -= 1;
                }
                '<' => stop_reading = true,
                '>' => stop_reading = false,
                _ => (),
            }
        } else if !skip_next_char {
            garbage_count = garbage_count + 1;
        }
        // reset skip_next_char back to false
        if skip_next_char {
            skip_next_char = false;
        }
    }
    println!();
    println!("Found score {}", score);
    println!("Found garbage count of {}", garbage_count);
}

fn read_string_from_file_to_vector(file_path: &str) -> io::Result<Vec<char>> {
    let mut f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    // let file = BufReader::new(&f);
    let mut river = String::new();
    f.read_to_string(&mut river)
        .expect("something went wrong reading the file");

    // println!("The river:\n{}", river);

    let mut vec = Vec::new();
    for c in river.chars() {
        vec.push(c);
    }
    Ok(vec)
}

fn string_to_character_vec(string: String) -> Vec<char> {
    let mut vec = Vec::new();
    for c in string.chars() {
        vec.push(c);
    }
    vec
}
