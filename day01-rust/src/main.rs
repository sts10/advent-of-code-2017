fn main() {
    println!("Let's do this");
    let input = "1122";
    println!("{}", input);
    let mut array_chars: [u32; 4] = [0; 4];
    let mut i: usize = 0;
    
    // build array_chars
    for c in input.chars() { 
        // println!("{:?}", c.to_digit(10));
        array_chars[i] = c.to_digit(10).unwrap();
        i = i + 1;
    }
    println!("{:?}", array_chars);

    let mut sum: u32 = 0;
    i = 0;
    for _ in array_chars.iter() {
        if i+1 < array_chars.len() && array_chars[i] == array_chars[i+1]{
            sum = sum + array_chars[i];
        }
        i = i + 1;
    }

    println!("{}", sum);
}
