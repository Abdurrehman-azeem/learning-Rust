use String;

fn main() {
    let s = "Hello String literal";
    let mut s_mutable = String::from(s);
    println!("unmodified string \n {}", s_mutable);

    s_mutable.push_str(" Adding this as well.");
    println!("Modified string \n {}", s_mutable);
}
