fn main() {
    conditions(4);
}

fn variables(){
    // Variables and Mutability
    let x = 5;
    // x = 6; //error

    let mut y = 10; // mutability
    y = 12;

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //constants (compile time)
}

fn shadowing_and_scope() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn data_types(){
    let i1 = -12;
    let i2: u32 = 32;
    let f1 = 1.23;
    let i3: u8 = 8;
    let i4: isize = 2;
    let b1: bool = true;
    let c1: char = 'z';

    // compound type
    let tup = (1, 2.34, 'x', false);
    let (i5, f2, c2, b2) = tup;
    let b3 = tup.3;


    let arr1 = [1, 3, 4];
    let arr2:[u32; 6] = [2; 6];
}

fn func(){
    println!("{}", func1(1));
    let x = {
        let y = func1(3);
        y + 3
    };
    println!("{}", x);
}

fn func1(x: i32) -> i32 { x + 1 }

fn conditions(x: i32){
    if x < 5 {
        println!("less")
    } else if x > 5 {
        println!("more");
    } else {
        println!("exact")
    }

    let s = if x >= 0 { "+ve" } else { "-ve" };
    for r in 1..=10 {
        println!("{}", r);
    }

    let mut i = 0;
    let i2 = loop {
        i += 1;
        if i >= 10 {
            break i;
        }
    };

    println!("{}", i2);
}