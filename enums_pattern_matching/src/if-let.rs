pub fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("the maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("the maximum is configured to be {max}");
    }
}
