fn main() {
    let x = 5;

    if x > 4 {
        println!("The number is greater than 4");
    } else {
        println!("The number is less than 4");
    }

    let y = if x == 5 { 10 } else { 2 };
    println!("The value of y is {y}");

    looped();
}

fn looped() {
    let mut counter = 0;

    let y = loop {
        counter += 1;
        if counter == 10 {
            break counter + 100;
        }
    };

    println!("The value for the counter is {y}");
}
