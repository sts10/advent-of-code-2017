
fn main() {
    println!("Ring number for 12 is {}", get_ring_number(12));
    println!("Ring number for 20 is {}", get_ring_number(20));
    println!("Ring number for 28 is {}", get_ring_number(28));
    println!("Ring number for 46 is {}", get_ring_number(46));
    println!("-----");
    println!("distance to closest railraod for 12 is {}", find_distance_to_closest_railroad(12));
}

fn get_ring_number(num: usize) -> i64 {
  let square_root: f64 = (num as f64).sqrt();
  // let lower_perfect = square_root as i64;
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

fn find_distance_to_closest_railroad(num: usize) -> usize {
    let square_root: f64 = (num as f64).sqrt();

    let upper_perfect: usize = (square_root as i64 + 1) as usize;
    println!("upper perfect for {} is {}", num, upper_perfect);

    let odd_perfect: usize;
    if upper_perfect % 2 != 0 {
        odd_perfect = upper_perfect;
    } else {
        odd_perfect = upper_perfect + 1;
    }
    let length_of_side: usize = odd_perfect;
    println!("odd perfect for {} is {}", num, odd_perfect);

    let last_of_row: usize = odd_perfect * odd_perfect;
    let first_of_row = (odd_perfect - 2) * (odd_perfect - 2) + 1;

    let railroad_4: usize = last_of_row - (length_of_side - 1)/ 2;
    let railroad_3: usize = railroad_4 - length_of_side + 1;
    let railroad_2: usize = railroad_3 - length_of_side + 1;
    let railroad_1: usize = railroad_2 - length_of_side + 1;

    // find which of the four railroads are closest to num
    if upper_perfect % 2 != 0{
        if num - railroad_3 >= num - railroad_4 {
            return num - railroad_4;
        } else {
            return num - railroad_3;
        }
    } else {
        if num - railroad_1 >= num - railroad_2 {
            return num - railroad_2;
        } else {
            return num - railroad_1;
        }
    }
}
// fn find_closest_square(num: usize) -> usize {
// }


