// Rust-101, Part 02: Generic types, Traits
// ========================================


// ## Generic datatypes
pub enum SomethingOrNothing<T>  {
    Something(T),
    Nothing,
}
// Instead of writing out all the variants, we can also just import them all at once.
pub use self::SomethingOrNothing::*;
type NumberOrNothing = SomethingOrNothing<i32>;

// ## Generic `impl` (impl<T> means for all type of SomethingOrNothing), 
impl<T> SomethingOrNothing<T> {
    // Static functions, like a constructor
    fn new(o: Option<T>) -> Self {
        match o {
            None => Nothing,
            Some(v) => Something(v)
        }
    }

    // method of SomethingOrNothing
    fn to_option(self) -> Option<T> {
        match self {
            Nothing => None,
            Something(v) => Some(v)
        }
    }
}

fn call_constructor(x: i32) -> SomethingOrNothing<i32> {
    SomethingOrNothing::new(Some(x))
}

// ## Traits
pub trait Minimum : Copy {
    fn min(self, b: Self) -> Self;
}

// Generic funcion over a type T with Minimun as trait
pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            Something(n) => e.min(n)
        });
    }
    min
}

// ## Trait implementations
impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

// **Exercise 02.1**: 
// Change your program such that it computes the minimum of a `Vec<f32>`
impl Minimum for f32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

// We again provide a `print` function.
impl NumberOrNothing {
    pub fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

impl SomethingOrNothing<f32> {
    pub fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

// Now we are ready to run our new code. Remember to change `main.rs` appropriately.
fn read_vec() -> Vec<f32> {
    // vec![18,5,7,3,9,27]
    vec![1.5, 5.2, 4.3, 4.4]
}
pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
