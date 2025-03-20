fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // let a = s; it will break code because s was moved to takes_ownership
    // println!("{s}"); it will break code because s was moved to takes_ownership

    let x = 5; // x comes into scope

    makes_copy(x); // because i32 implements the Copy trait,
                   // x does NOT move into the function,
    println!("{}", x); // so it's okay to use x afterward

    // } // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    // giving ownership and then taking it back

    let g1 = gives_ownership(); // gives_ownership moves its return
                                // value into g1
    let tmp_string = String::from("hello");

    let g2 = takes_and_gives_back(tmp_string); // tmp_string is moved into
                                               // takes_and_gives_back, which also
                                               // moves its return value into g2
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
