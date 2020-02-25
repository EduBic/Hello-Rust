// Rust-101, Part 00: Algebraic datatypes
// ======================================

enum NumberOrNothing {
    Number(i32),
    Nothing
}

// The following line tells Rust to take the constructors of NumberOrNothing 
// into the local namespace.
use self::NumberOrNothing::{Number,Nothing};

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    let mut min = Nothing;

    for el in vec {
        match min {
            Nothing     => min = Number(el),
            Number(n)   => min = Number(min_i32(n, el))
        }
    }
    return min;
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn read_vec() -> Vec<i32> {
    vec![18,5,7,1,9,27]
    // vec![]
}

fn print_number_or_nothing(n: NumberOrNothing) {
    match n {
        Nothing => println!("Nothing to print"),
        Number(n) => println!("min is {}", n)
    }
}


pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}

