### Control Flow

## If Expressions

Example

```
  let number = 5;
  
  if number > 6 {
    println!("The number is greater than 6");
  } else {
    println!("The number is less than 6");
  }
```

## Chaining multiple else if Together

If you have multiple conditions you want to check a variable for you can chain together multiple if else statements.

```
  let x = 10;
  
  if x == 8 {
    println!("x is 8");
  } else if x == 9 {
    println!("x is 9");
  } else if x == 10 {
    println!("x is 10");
  } else {
    println!("x didn't match any of the magic numbers");
  }
```

Rust if it maches with one if condition will skip all others.

## Using If With Let

**Remember** that numbers are evaluated as expressions. For example the number 5 evaluates to a value and is returned. Also, blocks of code evaluate to the last expression.

We can use the **if condition** as an expression in a let statement like so.

```
  let y = 10
  let x = if y == 10 { 2 } else { 5 };
  println!("the value of x is {x}");
```

For both arms in the let statement, you must make sure they are evaluated to the same value. Since the values are returned from the if statement blocks in the above code. You must make sure that they are of the same type. This is because Rust must know at compile time what type each variable is, having different types across if else blocks leads to different types for variables.

## Loops

There are 3 types of loops in Rust `loop`, `while` and `for`.

# Loop

loop continues repeating a block of code until explicitly told to stop. That is either through the `break` keyword or by issuing the `ctrl + c` command.

The way to run a loop is to make use of a `loop` keyword. 
The loop runs infinitely unless stopped using the `break` keyword.
You can return values by placing them after the `break` keyword.

An example is as follows.

```
    let mut counter = 0;

    let y = loop {
        counter += 1;
        if counter == 10 {
            break counter + 100;
        }
    };
```

You can also `return` from inside a loop, while break exists a loop, return exists the function the loop is running in.

You can Nest loops

```
  loop {
    println!("First loop");
    loop {
      println!("Second nested loop");
    }
  }
```

If you want to break out of the parent loop from within the nested loop you can make use of a labeled loop.

```
  let mut x = 10
  
  `parent_loop: loop {
    println!("parent loop");
    loop {
      x -= 1;
      println!("child loop");
      if x < 5 {
        break
      }
      if x < 4 {
        break 'parent_loop;
      }
    }
  }
```

The `break` statement breaks out of the child loop the `break \`parent_loop` statement preceding the label breaks out of the parent loop. Even through the execution is wihtin the child loop.
