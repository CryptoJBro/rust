fn main() {
    
    // if else control flow
    let x = 3;
    if x < 2 {
        println!("x is less than 2.");
    } else { // use else if for more options
        println!("x is greater than 2.");
    }
    
    // for loop (works on arrays only)
    for i in 0..4 { // last value excluded
        println!("{}", i); // prints 0 1 2 3
    }
    
    let mut y = 9; // mut since y needs to change
    while y > 5 { // as long as y is above 5 do this:
        println!("{}", y);
        y -= 1; // every loop, subtract 1 from y
    } // prints 9 8 7 6

    let x = 8;
    match x { // compares to the values on left side
        0..=7 => println!("from 0 to 7"), // use commas!
        8 => println!("x equals 8"),
        _ => println!("another value") // _ means default, no comma!
    }
}
