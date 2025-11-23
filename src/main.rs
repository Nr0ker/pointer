// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1); // & is a reference, it allowes to borrow the value without taking ownership of it
//     // so when we calculate_length(&s1); the value of s1 wont be dropped
//     // there is also a * which is caleed fereferencing and is an opposite of referencing 

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because s does not have ownership of what
//   // it refers to, the String is not dropped.

fn main() {
    let mut s = String::from("hello");

    change(&mut s); // if we want to modify the borrowed value we have to put the mut in the reference and also in the value itself

    let mut s = String::from("hello");

    // let r1 = &mut s;
    let r2 = &mut s; // and also we cannot have more than 1 mutable referenc to 1 value

    println!("{r1}, {r2}");

   

    // this code will throw an error if the r1 is not comented

    // data race is
    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There’s no mechanism being used to synchronize access to the data.

     // but as always we can do our lovely scope


    //  let mut s1 = String::from("hello");

    // {
    //     let r1 = &mut s1;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s1;

    // we can borrow multiple times, but if the borrowed varriable is borrowed more then 1 time AND is mutable, then it is an error

    // let r1 = &s1; // no problem
    // let r2 = &s1; // no problem
    // let r3 = &mut s1; // BIG PROBLEM

    // At any given time, you can have either one mutable reference or any number of immutable references.

// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope and is dropped, so its memory goes away.
//   // Danger!

//   // s is created inside of dangle, and after the scope has ended s is dropped, and there is nothing to reference to

// solution to that is to create the variable inside the function and then to return it
fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // ownership is moved out 

// if you have a reference to some data,
// the compiler will ensure that the data will not go out of scope before the reference to the data does.