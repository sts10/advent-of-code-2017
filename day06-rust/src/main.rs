fn main() {
    // let test:[i32; 4] = [0, 2, 7, 0];
    // println!("{:?}", reallocate(test));

    let mut current_bank:[i32; 16] = [2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14];
    let mut past_banks: Vec<[i32; 16]> = [[2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14]].to_vec();
    let mut number_of_cycles = 0;

    loop {
        current_bank = reallocate(current_bank);
        number_of_cycles = number_of_cycles + 1;
        let mut exit = false;
        let mut past_bank_number = 0; // for part 2
        for a_past_bank in &mut past_banks {
            past_bank_number = past_bank_number + 1;
            if current_bank == *a_past_bank {
                println!("FOUND IT");
                println!("bank is now {:?}", current_bank);
                println!("and that matches past bank #{}, which was {:?}", past_bank_number, *a_past_bank);
                println!("the number of cycles to get to the bank a second time: {}", number_of_cycles);
                println!("which happened at bank #{}", number_of_cycles + 1);
                // for part 2:
                println!("so number of cycles between matches is {}", number_of_cycles+1 - past_bank_number);
                exit = true;
                break;
            } 
        }
        if exit == true {
            break;
        } else {
            &past_banks.push(current_bank);
        }
    }
}

fn reallocate(mut bank: [i32; 16]) -> [i32; 16] {
    // find value and index of (first) max
    let mut value: i32 = find_max(bank);
    println!("max value is {}",value);
    let index_of_max: usize = find_index_of_max(bank);
    println!("index of max value is {}",index_of_max);
    // reassign selected bank spot to 0
    bank[index_of_max] = 0;
    // println!("bank is now {:?}", bank);
    // starting with next bank, drop 1 off into each bank till original value over
    let mut i: usize = index_of_max;
    while value > 0 {
        if bank.len() > i+1 {
            i = i + 1;
        } else {
            i = 0;
        }
        bank[i] = bank[i] + 1;
        value = value - 1;
        // println!("bank is now {:?}", bank);
    }
    // done with this reallocation
    return bank;
}

fn find_max(bank: [i32; 16]) -> i32 {
    return *bank.iter().max().unwrap();
}

fn find_index_of_max(bank: [i32; 16]) -> usize {
    let mut i = 0;
    for b in bank.iter() {
        if b == &find_max(bank) {
            break;
        }
        i = i + 1;
    }
    return i;
}
