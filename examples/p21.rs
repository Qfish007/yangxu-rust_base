fn main() {
    println!("Example p21: String");

    let _s = String::from("hello");
    let _s = String::new();
    let _s = "hello".to_string();
    let _s = "cbt";

    let mut s = String::from("hello");
    s.push_str(", world!");
    s.push('x');
    println!("{}", s);

    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    let s3 = s1 + &s2; // s1 is moved, s2 is not moved
    println!("s3: {}", s3);
    let s4 = &s3[0..5]; // s3 is not moved
    println!("s4: {}", s4);

    println!("s3: {:?}", s3.chars()); // ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!']
    println!("s3: {:?}", s3.bytes()); // [b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'!']
}
