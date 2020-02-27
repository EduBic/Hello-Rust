// Rust-101, Part 04: Ownership, Borrowing, References
// ===================================================

// ## Borrowing a shared reference

fn vec_min(v: &Vec<i32>) -> Option<i32> {
    use std::cmp;

    let mut min = None;
    // The method `iter` just borrows the vector it works on, 
    // and provides shared references to the elements. 'e : & i32'
    for e in v.iter() {
        // In the loop, `e` now has type `&i32`, so we have to dereference it to obtain an `i32`.
        min = Some(match min {
            None => *e,
            Some(n) => cmp::min(n, *e)
        });
    }
    min
}

// `vec_min` does not acquire ownership of the vector anymore, 
// we can call it multiple times on the same vector and also do things like
fn shared_ref_demo() {
    let v = vec![5,4,3,2,1];
    let first = &v[0];
    vec_min(&v);    // lend ownership to someone (share reference)
    vec_min(&v);
    println!("The first element is: {}", *first);
}

// ## Unique, mutable references

fn vec_inc(v: &mut Vec<i32>) {
    for e in v.iter_mut() {
        *e += 1;
    }
}

fn mutable_ref_demo() {
    let mut v = vec![5,4,3,2,1];
    vec_inc(&mut v);    // mutable reference, promise that nobady outside has access
    vec_inc(&mut v);    // duration of mut references must not overlap
}

// ## Summary
// The ownership and borrowing system of Rust enforces the following three rules:
// 
// * There is always exactly one owner of a piece of data
// * If there is an active mutable reference, then nobody else can have active access to the data
// * If there is an active shared reference, then every other active access to the data is also a
//   shared reference
// 

