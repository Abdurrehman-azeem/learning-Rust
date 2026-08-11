# Ownership

Ownership is unique to Rust and is one of its most unique features. It helps Rust make memory safety guarantees without needing a garbage collector.

Ownership is a set of rules that govern how memory is to be managed in a Rust program. If any of these rules are violated the program won't compile.

## Stack And Heap
In a systems programming language like Rust Stack and Heap are important concepts. A value existing on either stack or heap changes its behavior.

Both stack and heap are parts of memory available to the code at runtime for its use both structured in different ways. The stack stores values in an order, specifically LIFO. 

*All data on the stack must have known and fixed size*. 
*Data with an unknown size at compile time must be stored on the heap instead.*

The heap is less organized, when data is put on the heap the memory allocator finds an empty spot in the heap that is big enough marks is in use and returns a pointer which is the address of the location. This is called *allocating on the heap*.  

Note:- that since the size of the pointer is known it can be stored on the stack.

Pushing to the stack is faster than allocating on the heap. Since the memory allocator does not have to search and allocate memory. The location is always at the top of the stack.

Accessing data on the heap is generally slower than accessing data on the stack since you have to follow a pointer to get there. Contemporary processors are faster if they don't jump around in memory.

When your code calls a function the values passed into the function (including, potentially pointers to data on the heap) and the functions local variables get pushed onto the stack. They are all popped off the stack when the function returns.

Keeping tack of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap and cleaning up unused data on the heap so you don't run out of space are all problems that ownership addresses. 

Knowing ownership you won't have to think about stack or heap often. The main purpose of ownership is to manage data on the heap.

## Ownership Rules

1. Each value in Rust has an *owner*.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## The String Type

String type is stored on the heap and provides a good example of how Rust knows when to clean up the data from the heap.
In this section an emphasis on parts of String relating to ownership will be covered. These aspects apply to other complex types.

String literals like the one below are imutable thus not suitable for every usecase. 
```
  let s = "hello";
```

You can create a String type from the string literal using the `String::from` function,

```
  let mut s = String::from("Hello");
  s.push_str("hello");
```

The variable s is now extensible. s declared using the String type is mutable. Without `mut` you can not extend s.

## Memory and Allocation

In the case of a string literal (An example of string literal `"hello"`) the value is stored in the into the final executable. This is possible since the compiler knows what the content of a string literal are at compile time. This is possible due to the string literal's immutability.

With the `String` type since we don't know the amount of memory it would take ultimately (there could be additions to it like above). At compile time the compiler allocates memory on the heap at runtime.

1. Memory must be allocated at runtime.
2. We need to have a way to return the allocated memory when the string is no longer in use.

The first part is taken care of automatically. When we declared `s` in the code snippet above. The second part is difficult with most languages without a GC (garbage collector). We need to pair allocations with free operations.

In Rust as soon as we reach the end of the scope in which our String resides in, Rust calls the `drop` function. It is called automatically at the end of the closing curly bracket. 

## Variables and Data Interacting with Move

If you have two integer variables like so.

```
let x = 5;
let y = x;
```

x is declared and set to the value 5. y is declared and a copy of the value of x is bound to y. We have our two variables which exist on the stack. When the scope containing them ends they are simply removed from the stack. They are integers with a known fixed size which is why they are pushed to the stack.

If we look at a similar example concerning Strings. The behavior would be different due to the use of heap and the way strings are made up. You can look at the image below. (From [doc.rust-lang.org](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type))

```
  let s1 = String::from("hello");
  let s2 = s1;
```

![Rust String Type Data Structure](./ownership/images/string data structure.svg)

In the image you can see the String is made up of 3 parts a pointer, length and capacity. The pointer is to the memory that holds the contents of the string which exist on the heap. The pointer, length and capcity are all stored on the stack.

The length is how many bytes the contents of String are holding and the capicity is the total amount of bytes being stored by the contents.

When we assign s1 to s2, we copy the pointer, capcity and length from the s2 for s1 on the stack. We do not copy the data on the heap. So the pointer to both s1 and s2 point to the same area of the heap which contains the memory.

![Data structures for s1 and s2](./ownership/images/string data structure 2.svg)

The issue that we can face now is that since both s1 and s2 point to the same memory in the heap. If one of the variables goes out of scope. Rust automatically calls `drop` function and cleans the heap memory. While one of the variables still points to the area in memory which was cleaned. This is known as a *double free* error and is a memory safety bug.

Freeing memory twice can lead to memory corruption which can lead to security vulnerabilities.

Rust resolves this potential issue by making sure that upon declaration of s2. s1 is no longer valid. You can not use it and using it leads to a compilation error.

This should not be confused with the concept of a shallow copy. Even though the data isn't replicated and the pointer to memory location along with information about data (length, capacity) are  copied you may think that s2 is a shallow copy of s1. Except s1 becomes invalidated as soon as s2 is set to s1. So this operation is called `move`.

*A design choice implied by the above example Rust never creates deep copies of the data.*

Any automatic copies can be assumed to be inexpensive.

## Scope and Assignment

If we declare a string like so 

```
  let mut s = String::from("Hello");
```

and reassign the string 

```
  s = String::from("World");
```

Rust will immediately call the `drop` function and free the memory.


## Variables and Data Interacting with Clone

If we do want to deeply copy the heap data of the String and not just the stack data. We can do so by using the `clone` method.

```
  let s1 = String::from("Hello");
  let s2 = s1.clone();

  println!("s1 = {s1} and s2 = {s2}");
```
