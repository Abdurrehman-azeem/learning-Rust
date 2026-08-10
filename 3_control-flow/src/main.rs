fn main() {
    let x = 5;

    if x > 4 {
        println!("The number is greater than 4");
    } else {
        println!("The number is less than 4");
    }

    let y = if x == 5 { 10 } else { 2 };
    println!("The value of y is {y}");

    // looped();
    // println!("{}", disambiguous_loop());
    println!("{} fibonacci of 3", fibonacci_sequence(40));
}

// fn looped() {
//     let mut counter = 0;

//     let y = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter + 100;
//         }
//     };

//     println!("The value for the counter is {y}");
// }

// fn disambiguous_loop() -> i32 {
//     let mut count = 0;

//     'parent_loop: loop  {
//         println!("Parent  loop");
//         loop {
//             if count == 35 {
//                 println!("In child loop. \nCount is {}.", count);
//                 break 'parent_loop;
//             }
//             if count % 7 == 0 {
//                 count += 1;
//                 continue 'parent_loop;
//             }
//             count += 1;
//         }
//     }

//     let arr = [1,2,3,4,5,6,7];
//     let mut count1 = 0;

//     while count1 < 7 {
//         println!("Index is {} and array element is {}", count1, arr[count1]);
//         count1 += 1;
//     }

//     for num in arr  {
//         println!("Number in array {}", num);
//     }

//     for num in (1..1_000).rev() {
//         println!("{}...", num);
//     }

//     5
// }

fn fibonacci_sequence(mut n: isize) -> isize {
    let mut a = 0;
    let mut b = 1;
    let mut temp;

    if n == 0 || n == 1 {
        return n;
    } else {
        while n > 0 {
            temp = b;
            b = a + b;
            a = temp;
            n -= 1;
        }
    }
    return a
}
