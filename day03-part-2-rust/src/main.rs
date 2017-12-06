fn main(){
    // println!("value at index 6 is {}", get_value_at_index(6));
    // println!("value at index 10 is {}", get_value_at_index(10));
    // println!("value at index 16 is {}", get_value_at_index(16));
    println!("value at index 17 is {}", get_value_at_index(17));

    // let mut v: i64 = 0;
    // let mut i: i64 = 0;
    // while v < 347991 {
    //     v = get_value_at_index(i);
    //     i = i  +1;
    // }
    // println!("Solution is {}", get_value_at_index(i));
}

fn get_value_at_index(i: i64) -> i64 {
    if i == 1 { return 1; }
    if i == 2 { return 1; }
    if i == 3 { return 2; }
    if i == 4 { return 4; }
    if i == 5 { return 5; }
    if i == 6 { return 10; }
    if i == 7 { return 11; }
    if i == 8 { return 23; }
    if i == 9 { return 25; }

    let value: i64;
    println!("inside index is {}, with a value of {}", inside_index_of(i), get_value_at_index(inside_index_of(i)));
    
    // firstvalue of new ring
    if get_ring_length_from_index(i) > get_ring_length_from_index(i-1){
        value= get_value_at_index(i-1) 
             + get_value_at_index(i - get_ring_length_from_index(i-1));
    }

    // if top left corner
    else if is_perfect_even_square(i-1){
        value= get_value_at_index(inside_index_of(i) + get_value_at_index(i - 1));
    }

    // corner
    else if get_side_number_from_index(i) != get_side_number_from_index(i + 1){
        value= get_value_at_index(inside_index_of(i))
             + get_value_at_index(i - 1);
    }

    // square mark
    else if get_side_number_from_index(i + 1) != get_side_number_from_index(i + 2){
        value= get_value_at_index(i - 1)
             + get_value_at_index(inside_index_of(i))
             + get_value_at_index(inside_index_of(i-1));
    }

    // circle mark 
    else if (get_side_number_from_index(i - 1) != get_side_number_from_index(i)){
        value= get_value_at_index(i - 1)
             + get_value_at_index(i - 2)
             + get_value_at_index(inside_index_of(i))
             + get_value_at_index(inside_index_of(i+1));
    } else {
    value= get_value_at_index(i - 1)
         + get_value_at_index(inside_index_of(i))
         + get_value_at_index(inside_index_of(i+1))
         + get_value_at_index(inside_index_of(i-1));
    }

    println!("Found value at {} is {}", i, value);
    return value;
}

fn is_perfect_even_square(i: i64) -> bool {
    let square_root: f64 = (i as f64).sqrt();
    if square_root == (square_root.floor()) && square_root as i64 % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn is_perfect_odd_square(i: i64) -> bool {
    let square_root: f64 = (i as f64).sqrt();
    if square_root == (square_root.floor()) && square_root as i64 % 2 != 0 {
        return true;
    } else {
        return false;
    }
}

fn inside_index_of(i: i64) -> i64 {
    if i == 5 { return 1; }
    if i == 6 { return 1; }
    if i == 7 { return 1; }
    if i == 8 { return 1; }
    if i == 9 { return 1; }

    let ring_length = get_ring_length_from_index(i);
    let bottom_right_corner = get_index_of_bottom_right_corner_of_ring_from_index(i);
    let side_number = get_side_number_from_index(i);

    // i == 10 is first example of this
    if is_perfect_odd_square(i - 1) {
        return i - 1;
    }

    // bottom right corner
    if is_perfect_odd_square(i) {
        return i - ring_length;
    }

    let mut distance = ring_length - 9 + side_number*2;
    // corner
    if side_number != get_side_number_from_index(i + 1){
        distance = distance + 1;
    }
    // top left corner 
    // if is_perfect_odd_square(i + 1) {
        // return i - ring_length - 1;
    // }

    // standard X example. i == 11 first real example of this

    return i - distance;
}

fn get_ring_length_from_index(i: i64) -> i64 {
  if i == 1 { return 1; }
  if i <= 6 { return 8; }


  let square_root: f64 = (i as f64).sqrt();
  let upper_perfect: i64;
  if square_root == (square_root.floor()) {
      // it's an odd perfect square, so don't add 1
      upper_perfect = square_root as i64;
  } else {
      upper_perfect = square_root as i64 + 1;
  }

  // println!("upper perfect is {}", upper_perfect);

  if upper_perfect % 2 != 0{
      let ring_number = (upper_perfect - 1)/ 2;
      return ring_number * 8;
  } else {
      let ring_number = upper_perfect / 2;
      return ring_number * 8;
  }
}

fn get_index_of_bottom_right_corner_of_ring_from_index(i: i64) -> i64 {
    let square_root: f64 = (i as f64).sqrt();
    let upper_perfect: i64;
    if square_root.ceil() as i64 % 2 == 0 {
        upper_perfect = square_root.ceil() as i64 + 1; 
    } else {
        upper_perfect = square_root.ceil() as i64;
    }
    return upper_perfect * upper_perfect;
}

fn get_side_number_from_index(i: i64) -> i64 {
    let ring_length = get_ring_length_from_index(i);
    let bottom_right_corner_index = get_index_of_bottom_right_corner_of_ring_from_index(i);
    let first_index = bottom_right_corner_index - ring_length + 1;
    return ((i - first_index) * 4 / ring_length) + 1;
}

#[test]
fn can_determine_if_index_is_an_even_perfect_square() {
    assert_eq!(is_perfect_even_square(16), true);
    assert_eq!(is_perfect_even_square(25), false);
    assert_eq!(is_perfect_even_square(15), false);
}

#[test]
fn can_find_ring_length_given_index() {
    assert_eq!(get_ring_length_from_index(6), 8);
    assert_eq!(get_ring_length_from_index(2), 8);
    assert_eq!(get_ring_length_from_index(9), 8);
    assert_eq!(get_ring_length_from_index(11), 16);
    assert_eq!(get_ring_length_from_index(18), 16);
    assert_eq!(get_ring_length_from_index(27), 24);
}

#[test]
fn can_find_corners() {
    assert_eq!(get_side_number_from_index(17) != get_side_number_from_index(17 + 1), true);
    assert_eq!(get_side_number_from_index(21) != get_side_number_from_index(21 + 1), true);
    assert_eq!(get_side_number_from_index(20) != get_side_number_from_index(20 + 1), false);
    assert_eq!(get_side_number_from_index(22) != get_side_number_from_index(22 + 1), false);
}

#[test]
fn can_find_index_of_bottom_right_corner() {
    assert_eq!(get_index_of_bottom_right_corner_of_ring_from_index(7),9);
    assert_eq!(get_index_of_bottom_right_corner_of_ring_from_index(12),25);
    assert_eq!(get_index_of_bottom_right_corner_of_ring_from_index(17),25);
    assert_eq!(get_index_of_bottom_right_corner_of_ring_from_index(21),25);

    assert_eq!(get_index_of_bottom_right_corner_of_ring_from_index(16),25);
    assert_eq!(get_index_of_bottom_right_corner_of_ring_from_index(25),25);
    assert_eq!(get_index_of_bottom_right_corner_of_ring_from_index(49),49);
}

#[test]
fn can_find_side_number() {
    assert_eq!(get_side_number_from_index(11), 1);
    assert_eq!(get_side_number_from_index(15), 2);
    assert_eq!(get_side_number_from_index(18), 3);
    assert_eq!(get_side_number_from_index(19), 3);
    assert_eq!(get_side_number_from_index(23), 4);
}

#[test]
fn can_find_side_number_corner_cases() {
    assert_eq!(get_side_number_from_index(16), 2);
    assert_eq!(get_side_number_from_index(17), 2);
    assert_eq!(get_side_number_from_index(18), 3);
    assert_eq!(get_side_number_from_index(20), 3);
    assert_eq!(get_side_number_from_index(21), 3);
    assert_eq!(get_side_number_from_index(22), 4);
}

#[test]
fn can_find_inside_index_of_index(){
    assert_eq!(inside_index_of(14),3);
    assert_eq!(inside_index_of(23),8);
    assert_eq!(inside_index_of(19),6);
    
    assert_eq!(inside_index_of(13),3);
    assert_eq!(inside_index_of(22),7);
    assert_eq!(inside_index_of(49),25);
}
