# Programming concepts that are available in programming languages across the board

## This chapter goes over various concepts and their conventions when it comes to using them in Rust.

## Variables and Mutability

By default Rust encourages immutability. So that you can write your code in a way that favors easy concurrency.
There are times when mutability would be required.

__Note__ By default variables in Rust are immutable.

## Declaring a variable in Rust

You can declare a variable in Rust by using the `let` keyword. The Rust compiler makes sure any immutable value do not change.

For example
```
  let x = 5;
```

The above declared variable is immutable. For immutable variables we use `mut` keyword.

For example
```
  let mut x = 5;
```

We can reassign the variable like so

```
  x = 10;
```

## Declaring Constants

Much like immutable variables constants are not allowed to change and are tied to a name.
Constants and Variables differ. When declaring a constant you make use of the `const` keyword. 
You can not make use of the `mut` keyword with constants to make them mutable they are __always__ immutable.
Constants need to be anotated with the data type when they're declared.

Example of a constant

```
  const My_VARIABLE: u32 = 60 * 60 * 60; 
```

Constants can be declared in any scope.
Also, Constants differ from variables in one other way they may only be set to a constant expression like above.
  Not the result of a some computation at runtime.
Constants are named with all words in capital and underscores between each word.

## Shadowing

In Rust you can declare a variable with the same name as a variable already declared.

For example;

```
  let x = 30;
  println!("x: {x}");
  let x = 35;
  println!("x: {x}");
```

The redeclared variable __overshadows__ the previously declared variable of the same name. Until the scope it was declared in ends 
 
Example

```
    let new_variable = 3;
    let new_variable = 3 + new_variable; // original new_variable is shadowed here. replaced with 6
    println!("This is the original variable: {new_variable}"); // 6

    {
        let new_variable = 4;
        println!("This is the shadowed variable: {new_variable}"); // shadowed value is 4
    } // The inner scope ends and along with it the shadow

    println!("This is variable that was shadowed: {new_variable}"); // scope ends and the value is 6
```

You can also change the type of a value when shadowing a variable. Shadowing creates a new variable.
It is different from `mut` in the sense that you can use different variable types and that you don't need to declare new variables which requires coming up with new names. 

```
  let new_var = 10;
  let new_var = "hello";
```

## Data Types

Each value in Rust is of a certain data type.

## Data Types

Rust as you know is a statically typed language, meaning each variable's type must be declared. The compiler must know the type of the variable.
Usually the compiler can infer the type of the variable that is declared by the value we assign it or how we use the variable.
At times when the type is uncertain for example parsing string to either int or float, then we need to explicitly provide a type.

There are 2 main types of data type subsets.
Scalar and Compound

## Scalar Types

Rust has 4 main scalar types.

A scalar type represents one single value. Scalar types consist of integers, floating-point numbers, characters and Booleans.

# Integer Types

In Rust Integers can have multiple types 
8-bit signed and unsigned to 128-bit signed and unsigned.
Along with these we have the architecture dependent isize, usize types.

The isize and usize can be 32bits for a 32 bit architecture or 64 bits for a 64 bit architecture.

signed numbers can store a maximum of 
    -2^(n-1) to 2^(n-1) - 1
Unsigned numbers can store a maximum of
    0 to 2^n - 1

You can write the numbers in multiple types of forms. The integer literals that can be used are written below.

Decimal 10_000
  `let x = 10_000` 
  represents 10000

Hex 0xff
Octal 0o77
Binary 0b1111_0000
Byte(u8) b'A'

The integer default for Rust is i32.

# Integer Overflow

Lets say you have a u8 and you assign to it 256, one of two behaviors can occur.
When compiling in debug mode, the compiler will panic on the overflow.

When compiling the code for release with the --release flag. There are no checks for overflows.
  Rust on encountering overflows for the release build performs a `complement wrapping`.
  Where 256 wraps to become 0. 257 becomes 1 so on and so forth
    Basically take modulo of the overflow by 255 for e.g (257 % 255) + 1 =  2

Do remember though these overflows should be treated as errors. As this can lead to unexpected values and lead to scenarios where you get unexpected results.

# Floating-Point Types

Floating point numbers can be of 2 types f32 and f64, they are both unsigned. The default 
  type is f64 as it provides better precision and is just as fast, when it comes to operations, as f32 on modern cpus.


# Numeric Operations

Rust supports addition, multiplication, subtraction, division and remainder operations when it comes to numerical operations. 

When dividing ints the result is rounded down toward the nearest integer.

# The Boolean Type

Boolean takes up one byte of space and has 2 possible values true and false.

declaring a boolean

```
  let t = true;
  let f: bool = false;
```

# The Character Type
The most primitive of Rust's aplhabetic types.

Declared using a single quote.

```
  let c = 'z';
  let z: char = 'c';
```

A char takes up 4 bytes representing a unicode scalar value. Meaning it can represent much more than simple scalar values.

## Compound Types

Types that can compound multiple values into one type. We have 2 primitive compound types tuples and arrays.

# The Tuple Type

A tuple is a general way of grouping together a number of values with a variety of **types** into one compound type.

'''
  let tup = (500, 2.3, "hello");
'''

One way of getting the individual data of different types from the tuple is to deestructure the tuple through pattern matching.

```
  let (x, y, z) = tup
```

The values x, y, z correspond to the different values in side the tuple data structure.

The compiler first creates a tuple and binds it to the variable tup. Then it uses a pattern with let to destructure the tuple and assign the values to the variables.

Another way of accessing the embedded variables in the tuple is to make use of `.index` notation.

```
  let tup = (1, 2, 3.4, "hello");
  println!("Different values in the tuple, {tup.1}, {tup.2}, {tup.3}");
```

A tuple without any value is called a **unit**. It is represented like so. It is used to return an empty value or an empty return type.

```
  ()
```

# Array type

Another way to have a collection of vlaues of the same type. Unlike a tuple all the values in an array must have the same type. Arrays and Tuples both have a fixed length.

An example array below.

```
  let a = [1, 2, 3];
```

Arrays are useful when you wish to have your data available on the stack instead of the heap. Or if you wish to have the same length of array with same type.

A vector is another type, which is similar to the array, except that it can grow or shrink. Vector is provided by the standard library. A vectors content lives on the heap. You will often be faced with the decision to choose arrays or vectors, choose vectors if you are unsure. 

Declaring an array example

```
  let a: [i32; 5] = [1, 2, 3, 45, 5];
  let a = [3; 5];
```

The above snipped of code in the first line delcares an array of i32 values of size 5.

In the second line it declares an array of size 5 but with all the values set to 3.

You can access array elements like you would in any other language through indexes `a[0]` would fetch the first element `a[3]` would fetch the 4th element.

Obviously if you try to access an index that goes beyond the array's size, a runtime error would occur. Trying to access an index which is greater than the size of the array can lead to accessing invalid memory.

# Functions

The `main` function is the entry point of all Rust programs. Functions are declared using the `fn` keyword.
For Rust code we make use of the snake case as the conventional style for functions.

example of snakecase

```
  fn main() {
    another_function();
  }

  fn another_function() {}
```

Also you may have noticed that the function `main` calls `another_function` which is declared below the `main` function. 
Rust in practice does not care about the order of functions declared they can be before or after the function calling a function. `another_function` can be before the `main` function. They should just be defined in a scope that can be seen by the caller.

# Parameter

Functions can be defined to have parameters. Parameters are part of the function's signature. A parameter's type is called argument but both terms are used interchangably.

```
fn main() {
  another_function(12);
}

fn another_function(arg: i32) {
  println!("Hello, {arg}");
}
```

In function parameters you must provide anotations for the value (the type) that you are making use of. This is so that the compiler knows what type to expect from the variable across code.

# Statements and Expressions

Rust is an expression based language.

Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value.

Examples

The following is a statement.
```
  let x = 6;
```

The following is also a statement.
```
  fn another_function() {
    ...
  }
```

The following is an expression.

```
  5 + 6
```

The above expression evaluates to 11.

The following example includes expressions and statements.

```
  let x = {
    let y = 3;
    y + 1
  };
```

The above example's `let x = {...}` is a statement, the block `{...}` is a expression that evaluates it to 4.
Also, you may not that `y + 1` does not have a semi-colon. By not including a semi-colon we make the statement y + 1 into a statement, which means no value will be returned.

#  Functions With Return Values

Below is an example of functions with the return values.

```
  fn five -> i32 {
    5
  }

  fn main() {
    let fi = five();
    println!("The value is {fi}");
  }
```

You can return values from a function via the `return` keyword. The last expression in a function is implicitly returned.

```
  fn plus_one(x: i32) -> i32 {
    x + 1;
  }
```

The above code returns an error. The error would point to the fact that no value is being returned. Since we added a semi colon to the last expression `x + 1`. This addidition of a semi-colon makes it a statement. (A statement is not returned. To make it return we need to add a `return` keyword which performs the operation of returning a value.)

# Comments 

They are simple in Rust. Make use of the //, the idiotmatic way to provide comments is by  providing a comment on a line preceding the statement it attempts to provide information for. For multi-line comments you will have to write comments on each line separately.

## Control Flow

# If Expressions

Note that it is **If Expressions**, if statements evaluate to booleans.
