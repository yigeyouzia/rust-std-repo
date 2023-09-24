fn main() {
    let mut s = String::from("hello world");
    let index = first_world(&s);
    // s.clear();
    print!("{}", index);
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}