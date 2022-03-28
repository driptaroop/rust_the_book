pub fn invoke() {
    let mut s = String::from("hello");
    let size = gimme_length(&s);
    let s1 = &mut s; // only one mutable borrow possible at any point in time
    
    //let s2 = &mut s;
    //println!("{}, {}", s1, s2); // error since multiple mutable borrow not allowed
    
    // let s3 = &s; // error since immutable borrow is not possible if mutable borrow also exists 
    
    update_string(s1);
    println!("string: {}, size: {}", s, size);

    let s4 = &mut s;
    println!("{}", s4); // this works since there are no other mutable references in scope.
    // NOTE: scope of a variable ends when it is used last. so the scope of s1 already ended above. 
}

fn gimme_length(s: &String) -> usize {
    s.len()
}

fn update_string(s: &mut String) {
    s.push_str(" Dripto");
}

// fn reference_to_nothing() ->&String { // this is an error because the string gets dropped soon as the function finishes. so the reference is a reference to nothing.
  //  "hello" // ERROR: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// }