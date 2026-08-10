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
      if x % 3 {
        x -= 2
        continue 'parent_loop;
      }
    }
  }
```

The `break` statement breaks out of the child loop the `break 'parent_loop` statement preceding the label breaks out of the parent loop. Even through the execution is wihtin the child loop.

You can make use of `continue` statements on the parent loop through disambigating the loop. `continue 'parent_loop` the statement would allow us to rerun the parent loop.

# While Loop

Quicker to implement than loop with if else conditions and breaks. the `loop` keyword provides greater control ofcourse but this is more convenient.

```
  fn main()  {
    let mut count = 0;
    while count < 11 {
      count += 1;
    }
  }
```

# For Loop

Let's say you've got an array and you wish to iterate through it. 

```
  fn main() {
    let a = [1,2,3,4,5];
    let mut idx = 0;
    while idx < 5 {
      println!("Element {} at index {}", a[idx], idx);
      idx += 1;
    }
  }
```

Iterating through the array like so is *not efficient* and is *error prone*. Incase the array size changes (is reduced) you would have to update the loop if you do not then you could end up with an error.
This code is also *slow*, as the compiled code would have additional runtime check added to the code which check whether the index is out of bounds . This check would run on each iteration slowing down the program.


```
fn main() {
  let a = [1,2,3,4,5];

  for num in a {
    println!("{}", num);
  }
}
```

In this case you can use a for loop where the code is safer. Also, more efficient. The code generated via compilation can be more efficient as well as there is no need to compare the length of the array with the index on each iteration.

It should not surprise you but you can make use of a `for` loop for things better suited to `while` loops as well. 

For example, you can count down.

```
  fn main() {
    for num in (0..5).rev() {
      println!("{}...", num);
    }
  }
```

the .. operator is the range operator. 
`rev` is used to reverse the array.
