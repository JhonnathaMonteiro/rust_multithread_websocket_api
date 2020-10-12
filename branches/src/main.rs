fn main() {
    let number = 3;

    if number > 2 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let number2 = if condition {5} else {80};
    println!("The number2 value: {}", number2);

    // loops
    let mut number3 = 0;
    loop {
        number3 += 1;
        println!("Number3 value (loop): {}", number3);
        if number3 == 10 {
            break;
        }
    }

    number3 = 0;
    while number3 < 10 {
        number3 += 1;
        println!("Number3 value (while): {}", number3);
    }

    let a = [1, 2, 4, 6, 7];
    for elem in a.iter() {
        println!("Number in a (for in): {}", elem);
    }

    for number in (1..20).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let s = String::from("hello");// s comes into scope
    println!("{}",&s[0..2]);
    
    takes_ownership(s);// s's value moves into the function...
    // ... and so is no longer valid here
    
    // println!("{}", s); // error
    
    let x = 5;                 // x comes into scope

    makes_copy(x);    // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

  fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
