#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;

const MAX: usize = 100;

#[no_mangle]
fn main() -> i32 {
    let mut is_prime = [true; MAX + 1];

    println!("Prime numbers within 100 are as follows:");
    for i in 2..=MAX {
        if is_prime[i] {
            println!("{}", i);
            let mut multiple = i * 2;
            while multiple <= MAX {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    0
}
