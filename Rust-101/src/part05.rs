// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers

// Rust will generate an implementation of Clone that simply clones all the fields
#[derive(Clone)]
pub struct BigInt {
    pub data: Vec<u64>, // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0 // the most significant u64 must not be 0
        }
    }

    // We can convert any little-endian vector of digits (i.e., least-significant digit first) into
    // a number, by removing trailing zeros. The `mut` declaration for `v` here is just like the
    // one in `let mut ...`: We completely own `v`, but Rust still asks us to make our intention of
    // modifying it explicit. This `mut` is *not* part of the type of `from_vec` - the caller has
    // to give up ownership of `v` anyway, so they don't care anymore what you do to it.
    // 
    // **Exercise 05.1**: Implement this function.
    // 
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        if v.len() == 0 {
            BigInt { data: vec![] }
        } else {
            let mut res = BigInt::new(0);
            for e in v.iter().rev() {
                res.data.push(*e)
            }
            res
        }
    }

    // useless since we can use #[derive(Clone)] attribute of rust
    // pub fn clone(&self) -> Self {
    //     BigInt { data: self.data.clone() }
    // }

    pub fn count_digits(&self) -> u64 {
        let mut global_count = 0;
        for e in &self.data {
            let mut count = 0;
            let mut num = *e;

            while num != 0 {
                count += 1;
                num /= 10;
            }

            global_count += count
        }
        global_count
    }
}

// ## Cloning
fn clone_demo() {
    let v = vec![0,1 << 16];
    println!("{:?}", v);
    
    // move the ownership of v
    // let b1 = BigInt::from_vec(v); 

    // take an immutable ref, (clone() borrows v)
    // v.clone() == (&v).clone() // rust makes the conversion
    let b1 = BigInt::from_vec(v.clone());
    println!("{:?}", b1);

    // let b2 = BigInt::from_vec(v);
    // println!("{:?}", b2);

    let b3 = BigInt::from_vec(v);
    println!("{:?}", b3.count_digits());
}

// Again, Rust will generate this implementation automatically 
// if you add #[derive(Clone)] right before the definition of SomethingOrNothing.
use part02::{ SomethingOrNothing, Something, Nothing };

impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            // ref: we borrow n for the duration of the match arm
            Something(ref n) => Something(n.clone())   
        }
    }
}

// **Exercise 05.2**: Write some more functions on `BigInt`. What about a function that returns the
// number of digits? The number of non-zero digits? The smallest/largest digit? Of course, these
// should all take `self` as a shared reference (i.e., in borrowed form).

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}
fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */                                /* BAD! */
    *ptr = 1337;
}

pub fn main() {
    clone_demo()
}

