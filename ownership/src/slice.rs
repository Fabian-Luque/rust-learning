pub fn main() {
    let mut s = String::from("hello world");
    let word = first_word_bad(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word is now totally invalid!

    // String Slices
    let s = String::from("hello world");

    let hello = &s[0..5]; // reference to the first five characters of s
    let world = &s[6..11]; // reference to the next six characters of s
    let slice = &s[0..2];
    let slice = &s[..2]; // same as above
    let slice = &s[3..len];
    let slice = &s[3..]; // same as above
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..]; // same as above

    let mut s = String::from("hello world");

    let word = first_word(&s);
}

fn first_word_bad(sL: &String) -> usize {
    let bytes = s.as_bytes(); // convert s to array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        // iter is an iterator and enumerate is a method that returns a tuple
        if item == b' ' {
            // b' ' is the byte for space
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
  }
  &s[..];
}