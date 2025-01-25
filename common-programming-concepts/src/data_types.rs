pub fn main() {
  let guess: u32 = "42".parse().expect("Not a number!"); // if we don't specify the type, in this case, rust compiler will throw an error
                                                          // error[E0284]: type annotations needed
  println!("The guess is : {guess}");

  // Scalar types
  // Integers
  // 8-bit	  i8	  u8
  // 16-bit	  i16	  u16
  // 32-bit	  i32	  u32
  // 64-bit	  i64	  u64
  // 128-bit	i128	u128
  // arch	    isize	usize // depends on the architecture (32 or 64 bit)

  // Signed and unsigned refer to whether it’s possible for the number to be negative
  // Integers literals
  // Decimal	98_222
  // Hex	    0xff
  // Octal	  0o77
  // Binary	  0b1111_0000
  // Byte     (u8 only)	b'A'

  //Floating-Point Types (f32 and f64)
  let x - 2.0; // f64 - double precision
  let y: f32 = 3.0; // f32 - single precision

  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1

  // remainder
  let remainder = 43 % 5;

  // Booleans
  let t = true;
  let f : bool = false;

  // Characters
  let c = 'z';
  let z: char = 'ℤ';
  let heart_eyed_cat = '😻';

  // Compound Types (Tuples, Arrays)
  //Tuples
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // destructuring
  let (x, y, z) = tup;
  println!("The value of x is : {x}");
  let x: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
  // Arrays
  let a = [1,2,3,4,5];
  let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  // i32 is the type of the elements of the array
  // ; 5 is the length of the array
  let b = [3;5];
  // It will be [3,3,3,3,3]
  let first = a[0];
  let second = a[1];
  

}
