use std::fs::File;
use std::io;
use std::io::prelude::*;
// use std::io::BufRead;
// use std::io::BufReader;
// use std::str::FromStr;
fn main() {
    let file_name = "input.txt";
    let river: Vec<char> = read_string_from_file_to_vector(file_name).unwrap();

    let (score, garbage_count) = get_score_and_garbage_count(river);
    println!("score is {} and garbage is {:?}", score, garbage_count);
}
fn get_score_and_garbage_count(river: Vec<char>) -> (usize, usize) {
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
    (score, garbage_count)
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

#[test]
fn can_find_score_of_short_string() {
    let river = string_to_character_vec("{{{},{},{{}}}}".to_string());
    assert_eq!(get_score_and_garbage_count(river).0, 16);
}
#[test]
fn can_find_score_of_short_string_with_exclamation_marks() {
    let river = string_to_character_vec("{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string());
    assert_eq!(get_score_and_garbage_count(river).0, 9);
}
#[test]
fn can_find_garbage_count_of_short_string() {
    let river = string_to_character_vec("<random characters>".to_string());
    assert_eq!(get_score_and_garbage_count(river).1, 17);
}
#[test]
fn can_find_garbage_count_of_short_string_with_exclamation_point() {
    let river = string_to_character_vec("<{!>}>".to_string());
    assert_eq!(get_score_and_garbage_count(river).1, 2);
}
#[test]
fn can_find_garbage_count_of_long_string_with_exclamation_point() {
    let river = string_to_character_vec("<{o\"i!a,<{i<a>".to_string());
    assert_eq!(get_score_and_garbage_count(river).1, 10);
}

#[test]
fn can_find_score_of_file_input() {
    let file_name = "input.txt";
    let river: Vec<char> = read_string_from_file_to_vector(file_name).unwrap();
    assert_eq!(get_score_and_garbage_count(river).0, 8337);
}

#[test]
fn can_find_garbage_count_of_file_input() {
    let file_name = "input.txt";
    let river: Vec<char> = read_string_from_file_to_vector(file_name).unwrap();
    assert_eq!(get_score_and_garbage_count(river).1, 4330);
}
