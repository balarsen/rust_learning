// The current convention is to use the tests module to hold your
// "unit-style" tests. Anything that just tests one small bit of
// functionality makes sense to go here. But what about "integration-style"
// tests instead? For that, we have the tests directory.


//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        assert!(false);
    }

    #[test]
    #[should_panic]
    fn hello_world() {
        assert_eq!("Hello", "world");
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn hello_world2() {
        assert_eq!("Hello", "world");
    }

    #[test]
    fn add_two_test1() {
        assert_eq!(4, add_two(2));
    }

    // this is not run and shows up ignored in cargo test
    // unless cargo test -- --ignored which only tests the ignored ones
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}

// seems like private function must be tested this way
#[test]
fn test_add_three() {
    assert_eq!(5, add_three(2));
}


/// This function adds two to its argument.
///
/// # Examples
///
/// ```test
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
/// ```
/// use adder::add_two;
/// let x = add_two(5);
/// ```
pub fn add_two(a: i32) -> i32 {
    /// do the work
    let b = a + 2;
    return b;
}


/// This function adds three to its argument.
///
fn add_three(a: i32) -> i32 {
    a + 3
}
