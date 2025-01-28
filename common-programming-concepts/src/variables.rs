pub fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants (can't be mutated)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let z = 5;
    let z = z + 1;

    {
        // inner scope (block)
        let z = z * 2;
        println!("The value of z in the inner scope is : {z}");
    }

    println!("The value of z is : {z}");
}
