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

fn add_two(a: i32) -> i32 {
    a + 2
}


fn main() {
    println!("Hello, world!");
}
