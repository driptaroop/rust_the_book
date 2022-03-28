pub fn invoke(){
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..];
    println!("{} {}", hello, world);
}