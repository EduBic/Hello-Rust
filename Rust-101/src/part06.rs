// Rust-101, Part 06: Copy, Lifetimes
// ==================================

use part05::BigInt;

impl BigInt {
    fn min_try1(self, other: Self) -> Self {
        // assert remove when cargo build with --release flag
        debug_assert!(self.test_invariant() && other.test_invariant());
        // Now our assumption of having no trailing zeros comes in handy:
        // If the lengths of the two numbers differ, we already know which is larger.
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            // **Exercise 06.1**: data lens are equal
            self
        }
    }
}

fn vec_min(v: &Vec<BigInt>) -> Option<BigInt> {
    let mut min: Option<BigInt> = None; // default is None
    // If `v` is a shared reference to a vector, 
    // then the default for iterating over it is to call
    // `iter`, the iterator that borrows the elements.
    for e in v {
        let e = e.clone();
        min = Some(match {
            None => e,
            Some(n) => e.min_try1(n)
        })
    }
    min
}

// ## `Copy` types

use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Copy> Copy for SomethingOrNothing<T> {}

// implements Clone trait makes the copy constructur only explicit
// implements Copy trait makes the copy implicit (i32 impl it for ex.)

// ## Lifetimes

fn head<T>(v: &Vec<T>) -> Option<&T> {
    if v.len() > 0 {
        Some(&v[0]) // ref to the first element of v
    } else {
        None
    }
}

// C++ 
/*
  int foo(std::vector<int> v) {
    int *first = head(v);
    v.push_back(42);
    return *first;
  }
*/
// rust equivalent
fn rust_foo(mut v: Vec<i32>) -> i32 {
    // the scope of rust_foo is the lifetime of first
    let first: Option<&i32> = head(&v);
    /* v.push(42); */
    *first.unwrap()
}
