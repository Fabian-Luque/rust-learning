pub fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    //Statements and Expressions
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.
    let y = 6; // statement
               //let x = (let y = 6); // this does not compile
               // expressions
    let g = {
        let x = 3;
        x + 1 //  Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
    };

    println!("The value of g is: {g}");
    let h = five();
    println!("The value of h is: {h}");
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
