pub fn main() {
    let s1 = String::from("hello");
    // We call the action of creating a reference borrowing
    let len = calculate_length(&s1); // &s1 is a reference to s1 without taking ownership of it
                                     // s1 still exists because we have a reference to it
    println!("The length of '{s1}' is {len}");

    // Mutable references
    // We only one mutable reference at a time
    // let r1 = &mut s;
    // let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time

    let mut s = String:from("hello");
    change(&mut: s);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;


    // Combining mutable and immutable references
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM // cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{}, {}, and {}", r1, r2, r3);

    // VALID

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    //Dangling References
    let reference_to_nothing = dangle();

}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
  some_string.push_str(", world!");
}

fn dangle() -> &String { // dangle returns a reference to a String

  let s = String::from("hello"); // s is a new String

  // &s //BAD:  we return a reference to the String, s
  s // just return the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!