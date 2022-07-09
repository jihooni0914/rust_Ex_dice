use std::io;
use rand::{self, Rng};

const MAX_CASE : usize = 6 * 5 + 1;

fn set_usize_value() -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("invalid : try again");
                continue
            }
        };
    }
}

fn main() {

    println!("Enter the number of cases : ");
    let try_number : usize = set_usize_value();

    println!("Enter the number of times to throw the dice ( 1 ~ 5 ) : ");
    let throw_num : usize = loop {
        let input = set_usize_value();
        if input >= 1 && input <= 5 {
            break input;
        }
        else {
            println!("invalid range : try again");
        }
    };

    println!("n : {}, throw : {}", try_number, throw_num);
    
    
    let mut dice_sum_result : [u32 ; MAX_CASE] = [0 ; MAX_CASE];
    for _i in 0..try_number {
        let mut sum = 0;
        for _j in 1..=throw_num {
            sum += rand::thread_rng().gen_range(1..=6);
        }

        dice_sum_result[sum] += 1;
    }

    let max: usize = throw_num * 6;
    for i in throw_num..=max {
        println!("#{}\t:\t{}", i, dice_sum_result[i]);
    }

}
