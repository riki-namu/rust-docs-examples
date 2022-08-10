fn main() {
    println!("Hello, world!");

    // 1.basics
    {

        let s = "hello";
        println!("{}",s);
        //scope is over, s no longer valid
    }
    // cannot print s since scope is over
    // println!("{}",s);

    // 2.ownership availability through copied type
    {
        let s1 = String::from("hello");
        let s2 = s1;
        // s1 unavailable through shallow copy
        // println!("{}, world!", s1);

        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // 3.in case of integer type
    {
        // valid for integer types
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
    }

    // 4.ownership availability over functions
    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
        //println!("{}",s); -> won't work

        let x = 5;                      // x comes into scope

        makes_copy(x);                  // x would move into the function,

        println!("{}",x);
    }

    {
        let s1 = gives_ownership();         // gives_ownership moves its return
                                            // value into s1

        let s2 = String::from("hello");     // s2 comes into scope

        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3

        println!("{}",s1);
        //println!("{}",s2); -> won't work
        println!("{}",s3);
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {            // gives_ownership will move its
                                            // return value into the function
                                            // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                             // some_string is returned and
                                            // moves out to the calling
                                            // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}