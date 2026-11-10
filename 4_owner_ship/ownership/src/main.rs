use String;
use std::io::{self, Write}

fn main() {
    // Simple program that takes a  string literal
    // creates a mutable String variable from it
    // then appends to it
    // finally prints it.
    // let s = "Hello";
    // let mut s = String::from(s);
    // s.push_str(", World!");

    // println!("{:?}", s);

    let s = String::from("hello");
    move_func(s); // s is moved here and thus ownership is transferred to the move_function.
    let my_bool = true;
    copy_func(my_bool);
    println!("I can still use my copied value {my_bool}");

    // a simple snippet of code that shows how transfer of ownership works.
    let x = String::from("hello ownership");
    let y = transfer_and_return_ownership(x); // This function takes ownership of x
    // x can no longer be used since it was moved.
    // The ownership is then returned to y. x remains invalid
    println!("Ownership has been transfered. {y}");
    let (area, circumference) = area_and_circumference(32);
    println!("{}, {}", area, circumference);

    testing_mutable_references();
    testing_out_slices();
    // let data = get_first_word_from_user_input();
} // my_bool goes out of scope here.

fn transfer_and_return_ownership(s: String) -> String {
    println!("{s}");
    s
}

fn move_func(s: String) {
    println!("This value was moved {s}");
} // s is dropped here.The memory is released by the allocator. Drop is called here.
  // if the variable had been returned the ownership would have transferred to the
  // function the variable was returned to.

fn copy_func(b: bool) {
    println!("This value was copied {b}");
}

fn area_and_circumference(radius: i32) -> (f32, f32) {
    (2.0*3.142*(radius as f32), 3.142*(radius as f32)*(radius as f32))
}

fn testing_mutable_references() {
    let s1 = 10;
    let s2 = &s1;

    println!("{s2} the value of a non-mutable reference");
    // *s2 = 11; This won't work since it is a non-mutable refernce.
    //
    let s3 = 22;
    // let s4 = &mut s3; This would throw an error as s3 is not mutable we cannot
    // create a mutable reference from it.
    let mut s5 = 55;
    let s6 = &mut s5;
    println!("Value before modification {s6}");
    *s6 = 1 + 55;
    println!("Value after modification {s6}");
}

fn testing_out_slices() {
    let text = String::from("slicess");

    let sli = &text[0..3];
    let icess = &text[3..7];

    println!("{sli}  and {icess}");
}

// fn get_first_word_from_user_input() -> &str {
//     let mut s = String::new();

//     println!("Input text.");
//     io::stdout().flush().expect("Failed to flush stdout.");

//     io::stdin()
//         .read_line(&mut s)
//         .expect("Error reading input from stdin.");

//     let s_bytes = s.as_bytes();
//     for (i, &char) in s_bytes.iter().enumerate() {
//         if char == b' ' {
//             return &s[..i];
//         }
//     }

//     println!("String contains single word.")
//     return &s[..];
// }
