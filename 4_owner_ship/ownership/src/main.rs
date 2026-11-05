use String;

fn main() {
    // Simple program that takes a  string literal
    // creates a mutable String variable from it
    // then appends to it
    // finally prints it.
    let s = "Hello";
    let mut s = String::from(s);
    s.push_str(", World!");

    println!("{:?}", s);
}
