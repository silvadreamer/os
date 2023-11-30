#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;

const MAX_NUM: usize = 1000;

#[no_mangle]
fn main() -> i32 {
    for num in 2..=MAX_NUM {
        if is_prime(num) {
            println!("{}", num);
        }
    }
    0
}

fn is_prime(num: usize) -> bool {
    if num < 2 {
        return false;
    }

    for i in 2..num {
        if i * i > num {
            break;
        }
        if num % i == 0 {
            return false;
        }
    }

    true
}
