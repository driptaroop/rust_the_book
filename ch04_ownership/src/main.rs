/**
# Ownership Rules
    * Each value in Rust has a variable that’s called its owner.
    * There can only be one owner at a time.
    * When the owner goes out of scope, the value will be dropped.
 */
mod references;
mod slices;

fn main() {
    copy_vs_move();
    references::invoke();
    slices::invoke();
}

fn copy_vs_move(){
    let x1 = 5; // x1 is i32 type created in stack. stacks are allocated in compile time.
    let x2 = x1; // x2 also created in stack. atm x1 and x2 both exists. this is a copy since i32 implements copy trait.

    println!("x1: {}, x2: {}", x1, x2);

    let s1 = String::from("hello"); // s1 created in heap. so allocated in the runtime.
    let s2 = s1; // this is a move and NOT a shallow copy

    // println!("s1: {}", s1); // error because s1 doesn't exist now since the value is moved
    println!("s2: {}", s2);
}

fn ownership(){
    let s: String = String::from("hello");
    let i: i32 = 1;
    takes_ownership(s);
    makes_copy(i);
    // println!("{}", s);
    println!("{}", i);

    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1);
    println!("{}", s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn takes_and_gives_back(some_str: String) -> String {
    some_str
}