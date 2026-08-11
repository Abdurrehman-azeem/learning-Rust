use String;

fn main() {
    let s = "Hello String literal";
    let mut s_mutable = String::from(s);
    println!("unmodified string \n {}", s_mutable);

    s_mutable.push_str(" Adding this as well.");
    println!("Modified string \n {}", s_mutable);

    // scope_and_drop_exercise();
    cloning_complex_variables();
}

// fn scope_and_drop_exercise() {
//     let mut s1 = String::from_str("Hello");
//     println!("Is s1 valid? {:?}", s1);
//     let s2 = s1;
//     println!("Is s1 still valid? {:?}", s1);
// }

fn cloning_complex_variables() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {s1} a nd s2 = {s2}");
}
