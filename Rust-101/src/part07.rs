// Rust-101, Part 07: Operator Overloading, Tests, Formatting
// ==========================================================

pub use part05::BigInt;

// With our new knowledge of lifetimes, we are now able to write down the desired type of `min`:
pub trait Minimum {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

pub fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
    let mut min: Option<&T> = None;
    for e in v {
        min = Some(match min {
            None => e,
            Some(n) => n.min(e)
        });
    }
    min
}

// **Exercise 07.1**: For our `vec_min` to be usable with `BigInt`, you will have to provide an
// implementation of `Minimum`. You should be able to pretty much copy the code you wrote for
// exercise 06.1. You should *not* make any copies of `BigInt`!
impl Minimum for BigInt {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            // compare back-to-front, i.e., most significant digit first
            let mut idx = self.data.len();
            while idx > 0 {
                idx = idx-1;
                if self.data[idx] < other.data[idx] {
                    return self;
                } else if self.data[idx] > other.data[idx] {
                    return other;
                }
            }
            // the two are equal
            return self;
        }
    }
}

// ## Operator Overloading
// N.B. the attribute #[derive(PartialEq)] does automatically this implementation
impl PartialEq for BigInt {
    #[inline]
    fn eq(&self, other: &BigInt) -> bool {
        debug_assert!(self.test_invariant() && other.test_invariant());
        self.data == other.data
    }
}


// Now we can compare `BigInt`s. Rust treats `PartialEq` special in that it is wired to the operator
// `==`:
fn compare_big_ints() {
    let b1 = BigInt::new(13);
    let b2 = BigInt::new(37);
    println!("b1 == b1: {} ; b1 == b2: {}; b1 != b2: {}", b1 == b1, b1 == b2, b1 != b2);
}

// ## Testing
// With our equality test written, we are now ready to write our first testcase.
#[test]
fn test_min() {
    let b1 = BigInt::new(1);
    let b2 = BigInt::new(42);
    let b3 = BigInt::from_vec(vec![0, 1]);

    assert_eq!(*b1.min(&b2), b1);
    assert_eq!(*b3.min(&b2), b2);
}

// ## Formatting

// All formating is handled by [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html). I won't
// explain all the details, and refer you to the documentation instead.
use std::fmt;

// N.B. Automatically created with #[derive(Debug)]
impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}


#[test]
fn test_vec_min() {
    let b1 = BigInt::new(1);
    let b2 = BigInt::new(42);
    let b3 = BigInt::from_vec(vec![0, 1]);

    let v1 = vec![b2.clone(), b1.clone(), b3.clone()];
    let v2 = vec![b2.clone(), b3.clone()];
    
    assert_eq!(vec_min(&v1), Some(&b1));
    assert_eq!(vec_min(&v2), Some(&b2));
}

// **Exercise 07.1**: Add some more testcases. In particular, make sure you test the behavior of
// `vec_min` on an empty vector. Also add tests for `BigInt::from_vec` (in particular, removing
// trailing zeros). Finally, break one of your functions in a subtle way and watch the test fail.

// **Exercise 07.2**: Go back to your good ol' `SomethingOrNothing`, and implement `Display` for it.
// (This will, of course, need a `Display` bound on `T`.) Then you should be able to use them with
// `println!` just like you do with numbers, and get rid of the inherent functions to print
// `SomethingOrNothing<i32>` and `SomethingOrNothing<f32>`.

