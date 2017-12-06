struct Point {
    index: i64,
    length_of_side: i64,
    pos_on_side: i64,
    shape: String,
    num: i64,
}
fn main() {
    let point3 = Point {
        index: 3, 
        length_of_side: 3,
        pos_on_side: 3,
        shape: String::from("triangle"),
        num: 2,
    };
    let mut spiral = Vec::new();
    spiral.push(point3);

}

fn build_next_point(spiral: &Vec<Point>) -> Point {
    let last_point: &Point = &spiral[spiral.len()-1];
    let mut new_point = Point {
        index: last_point.index +1,
        length_of_side: get_length_of_side(&spiral),
        pos_on_side: get_pos_on_side(&spiral),
        shape: get_shape(&spiral),
        num: get_num(&spiral),
    };

    return new_point;
}

fn get_length_of_side(&spiral) -> i64{
    let last_point: &Point = &spiral[spiral.len()-1];
    // let ring_number = ???
    // return ring_number* 2 + 1;
}
