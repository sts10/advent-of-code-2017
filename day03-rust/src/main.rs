
fn main() {
    println!("Ring number for 12 is {}", get_ring_number(12));
    println!("distance to closest railroad for 12 is {}", find_distance_to_closest_railroad(12));
    println!("total steps for 12 is {}", find_distance_to_closest_railroad(12) + get_ring_number(12));
    
    println!("total steps for 23 is {}", find_distance_to_closest_railroad(23) + get_ring_number(23));
    println!("total steps for 1024 is {}", find_distance_to_closest_railroad(1024) + get_ring_number(1024));
    println!("total steps for 347991 is {}", find_distance_to_closest_railroad(347991) + get_ring_number(347991));
}

fn get_ring_number(num: usize) -> i64 {
  let square_root: f64 = (num as f64).sqrt();
  let upper_perfect = square_root as i64 + 1;

  println!("upper perfect is {}", upper_perfect);

  if upper_perfect % 2 != 0{
      let ring_number = (upper_perfect - 1)/ 2;
      return ring_number;
  } else {
      let ring_number = upper_perfect / 2;
      return ring_number;
  }
}

fn find_distance_to_closest_railroad(num: i64) -> i64 {
    let square_root: f64 = (num as f64).sqrt();

    let upper_perfect: i64 = (square_root as i64 + 1) as i64;
    println!("upper perfect for {} is {}", num, upper_perfect);

    let odd_perfect: i64;
    if upper_perfect % 2 != 0 {
        odd_perfect = upper_perfect;
    } else {
        odd_perfect = upper_perfect + 1;
    }
    let length_of_side: i64 = odd_perfect;
    println!("odd perfect for {} is {}", num, odd_perfect);

    let last_of_row: i64 = odd_perfect * odd_perfect;
    let first_of_row = (odd_perfect - 2) * (odd_perfect - 2) + 1;

    let railroad_4: i64 = last_of_row - (length_of_side - 1)/ 2;
    let railroad_3: i64 = railroad_4 - length_of_side + 1;
    let railroad_2: i64 = railroad_3 - length_of_side + 1;
    let railroad_1: i64 = railroad_2 - length_of_side + 1;

    // find which of the four railroads are closest to num
    if upper_perfect % 2 != 0{
        if (num - railroad_3).abs() >= (num - railroad_4).abs() {
            return (num - railroad_4).abs();
        } else {
            return (num - railroad_3).abs();
        }
    } else {
        println!("railroad 1 is {}, railroad 2 is {}, and num is {}", railroad_1, railroad_2, num);
        if (num - railroad_1).abs() >= (num - railroad_2).abs() {
            return (num - railroad_2).abs();
        } else {
            return (num - railroad_1).abs();
        }
    }
}

#[test]
fn can_find_ring_number() {
    assert_eq!(get_ring_number(12), 2);
    assert_eq!(get_ring_number(21), 2);
    assert_eq!(get_ring_number(26), 3);
    assert_eq!(get_ring_number(29), 3);
}

#[test]
fn can_find_ring_number_of_perfect_squares() {
    assert_eq!(get_ring_number(16), 2);
    assert_eq!(get_ring_number(25), 2);
}
#[test]
fn can_find_distance_to_closest_railroad() {
    assert_eq!(find_distance_to_closest_railroad(12), 1);
    assert_eq!(find_distance_to_closest_railroad(21), 2);
}
#[test]
fn can_find_distance_to_closest_railroad_of_perfect_squares() {
    assert_eq!(find_distance_to_closest_railroad(16), 1);
    assert_eq!(find_distance_to_closest_railroad(25), 2);
}
