

fn main() {
    // let x = 10;
    // println!("The value for x is: {x}");
    // x = 6; // Should throw an error.
    // println!("The value for x is: {x}");

    let mut x = 10;
    println!("The value for x is: {x}");
    x = 5;
    println!("The new value for x is: {x}");


    // Scoping shadowed variables
    let new_variable = 3;
    let new_variable = 3 + new_variable;
    println!("This is the original variable: {new_variable}");

    {
        let new_variable = 4;
        println!("This is the shadowed variable: {new_variable}");
    }

    println!("This is variable that was shadowed: {new_variable}");

    let parsed_variable: u32 = "42".parse().expect("Not a number.");
    println!("This is the parsed variable {parsed_variable}.");


    let mut overflow_var: u8 = 0;
    overflow_var = overflow_var + 128 + 129;
    println!("This is the wrapped variable {overflow_var}");

    let truncated_result: u8 = 9 / 5; // The result is 4 even though the actual value is 1.8
    println!("This is the resultant truncated value for the divison. {truncated_result}");

    let fi = five();
    println!("The value returned from the function five, {fi}");
}

fn five() -> i32 {
    5
}
