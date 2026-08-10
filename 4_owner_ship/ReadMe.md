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
  let s = String::from("Hello");
```

The variable s is now extensible. s declared using the String type is mutable.
