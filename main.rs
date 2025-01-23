// fn main() {
//     let is_it_fuc: bool = true;
//     // i32 -> signed integer of 32bits
//     // signed can hold positive and negatives values
//     let num: i32 = 10;

//     // u32 -> 1ˆ32 -1
//     // u8 -> unsigned integer of 8bits
//     // only can hold positive values
//     // 2ˆ8 -1 = 255
//     let small_number: u8 = 255;

//     // -2ˆ7 -> 2ˆ7 -1
//     // -128 -> 127
//     let small_number_2: i8 = -128; // 127

//     //Operating system related type 32bits or 64bits
//     let sys_num: isize = -10;
//     let sys_num_2: usize = 10;
// }

fn main() {
    // let custom_num = 98_000; // 98000
    // let hex_num = 0xfa;
    // let bin_num = 0b0010_1011;
    // let byte_num = b'A';

    // println!("{}", custom_num); // 98000
    // println!("{}", hex_num); // 250
    // println!("{}", bin_num); // 43
    // println!("{}", byte_num); // 65

    // Primitive Types
    let float_num: f32 = 3.14;
    let float_num_2: f64 = 3.23122414123;

    let tup: (i32, &str, u8) = (20, "hello", 1);
    println!("{}", tup.1);

    let (a, b, c) = tup;
    println!("{}", c);

    let x = [1, 2, 3, 4, 5];
    println!("{}", x[3]);

    let y = [2; 6]; // 6-element array of 2 [2, 2, 2, 2, 2, 2]
    println!("{}", y[5]);
}
