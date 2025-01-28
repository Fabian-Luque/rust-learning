fn main() {
    // Ownership rules
    //      Each value in Rust has an owner.
    //      There can only be one owner at a time.
    //      When the owner goes out of scope, the value will be dropped.

    // Variable Scope
    // let s = 'hello'; // s is valid from this point forward
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    //Variables and Data Interacting with Move
    let x = 5;
    let y = x; // valid with integers

    let s1 = String::from("hello");
    let s2 = s1; // s1 is no longer valid, s2 is valid (rust will remove s1 from memory)

    // Scope and Assignment

    let mut s = String::from("hello");
    s = String::from("ahoy"); // the previous value of s is dropped

    println!("{s}, world!");

    // Variables and Data Interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 still exists

    println!("s1 = {s1}, s2 = {s2}");

    //Ownership and Functions

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    takes_ownership(s);

    makes_copy(x);

    //Return Values and Scope
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
