// The current convention is to use the tests module to hold your
// "unit-style" tests. Anything that just tests one small bit of
// functionality makes sense to go here. But what about "integration-style"
// tests instead? For that, we have the tests directory.

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

pub fn add_two(a: i32) -> i32 {
    a + 2
}
