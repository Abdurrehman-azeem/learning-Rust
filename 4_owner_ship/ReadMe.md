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

Keeping track of what parts of the code are using what data on the heap, minimizing the amount of duplicate data and cleaning up unused data on the heap so you don't run out of space are all problems that ownership addresses. 

Knowing ownership you won't have to think about stack or heap often. The main purpose of ownership is to manage data on the heap.

## Ownership Rules

1. Each value in Rust has an *owner*.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

```
  {
    let s = String::from("hello");
  } // s is the owner of "hello" and s goes out of scope here.
  // The allocated data for s is recovered at the end of the scope with `drop` function.
```

## The String Type

String type is stored on the heap and provides a good example of how Rust knows when to clean up the data from the heap.
In this section an emphasis on parts of String relating to ownership will be covered. These aspects apply to other complex types.

String literals like the one below are imutable thus not suitable for every usecase.
They are also not stored on the heap they are embedded directly into the code.
```
  let s = "hello";
```

You can create a String type from the string literal using the `String::from` function.

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

![Rust String Data structure](https://doc.rust-lang.org/book/img/trpl04-01.svg)

In the image you can see the String is made up of 3 parts a pointer, length and capacity. The pointer is to the memory that holds the contents of the string which exist on the heap. The pointer, length and capcity are all stored on the stack.

The length is how many bytes the contents of String are holding and the capicity is the total amount of bytes being stored by the contents.

When we assign s1 to s2, we copy the pointer, capcity and length from the s2 for s1 on the stack. We do not copy the data on the heap. So the pointer to both s1 and s2 point to the same area of the heap which contains the memory.

![Data structures for s1 and s2](https://doc.rust-lang.org/book/img/trpl04-02.svg)

The issue that we can face now is that since both s1 and s2 point to the same memory in the heap. If one of the variables goes out of scope. Rust automatically calls `drop` function and cleans the heap memory. While one of the variables still points to the area in memory which was cleaned. This is known as a *double free* error and is a memory safety bug.

Freeing memory twice can lead to memory corruption which can lead to security vulnerabilities.

Rust resolves this potential issue by making sure that upon declaration of s2. s1 is no longer valid. You can not use it and using it leads to a compilation error.

This should not be confused with the concept of a shallow copy. Even though the data isn't replicated and the pointer to memory location along with information about data (length, capacity) are  copied you may think that s2 is a shallow copy of s1. Except s1 becomes invalidated as soon as s2 is set to s1. So this operation where the data on the heap isn't copied but the pointer, length and capacity are is called `move`.

In the above example s1 was moved into s2.

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

Rust will immediately call the `drop` function and free the memory. Since the original memory allocated in the heap has nothing pointing to it.

![Assignment Example](https://doc.rust-lang.org/book/img/trpl04-05.svg)

## Variables and Data Interacting with Clone

If we do want to deeply copy the heap data of the String and not just the stack data. We can do so by using the `clone` method.

Also  note that when you clone the data you need to take care of the fact that cloning large data on the heap can be a slow process.

```
  let s1 = String::from("Hello");
  let s2 = s1.clone();

  println!("s1 = {s1} and s2 = {s2}");
```

## Stack-Only Data Copy

As you see below the integers below are both copied. They do not need the clone function. This is because their size is known and they are on the stack.

```
  let x = 10;
  let y = x;
```

Every type that is stored on the stack has a `trait` (will be discussed in chapter 10.) The `Copy` trait. Values that implement the `copy trait` are NOT `moved`. They remain valid after they are assigned to another variable. 

If a type or any of its parts `implements` the `drop trait` we can't annotate it with the `copy` `trait`.

## Ownership & Functions

The mechanics of passing a value to a function can be described as either a `move` or `copy` operation.

## Return Values & Scope

Returning values can also transfer ownership.
The ownership of a variable follows a pattern when the variable is transfered (x=y), ownership is moved to the new variable. When the variable includes data on the heap. If the variable goes out of scope the data on the heap is cleaned via `drop`. If ownership has been `moved` to another variable `drop` is not called.

## References & Borrowing
A reference is like a pointer. It is an address that we can follow to access data. That data is owned by some other variable. Unlike a pointer a reference is *guaranteed* to point to a valid value for the life of the reference.

References allow you to refer to some value without taking ownership of that value.

```
  let s1 = String::from("hello");
  let l1 = calculate_length(&s1);

  println!("Length of string is {l1}");

  func calculate_length(s: &String) -> usize {
    s.len()
  }
```

![References Visualised](https://doc.rust-lang.org/book/img/trpl04-06.svg)


We can dereference with the `*` operator. It is the opposite of references.

Also  notice since `&s1` is a reference passed to the function. `&s1` does not own the data. It is a reference to s1. So when it goes out of scope the reference is dropped and not the data associated with s1.

The action of creating a reference is called `borrowing`. The ownership stays the same.

If you try to modify a value pointed to by a reference you can't unless it is a `mutable reference`.

## Mutable Reference

Like variables references are also immutable by default.
We can only make mutable references from mutable variables. Trying to make a mutable reference from an immutable variable results in an error.

The below code snippet is valid. Since the variable the mutable reference is created against is mutable.

```
  let mut s1 = 10;
  let s1_ref = &mut s1;
```

Mutable references have one big restriction. If a value has multiple mutable references the references will fail.

```
  let mut s = 10_000;

  let r1 = &mut s; // This is valid
  let r2 = &mut s; // This is not valid and will fail.
```

This is designed to allow mutability in a very controlled fashion.
This can help prevent data races at `compile time`.

Data rances can stem from the following actions.
1. 2 or more pointers access the same data at the same time.
2. Atleast one of the pointers is being used to write the data.
3. There is no mechanism being used to synchronize access to the data.

Rust also prevents combining mutable and immutable references. You cannot have immutable and mutable references simultaneously.

```
  let mut s = String::from("Hello");
  
  let r1 = &s;
  let r2 = &s;
  let  r3 = &mut s;
```

The above code throws an error, since you can have multiple mutable references having a immutable reference while mutables references exist causes a compile time error.
This is to prevent data from being modified when it is expected to stay the same.

We can however create a mutable reference after the immutable references have been used.
A reference's scope starts from where it is introduced and continues to where it is last used. If you declare an immutable reference after the last usage of an immutable reference you will face no issues.

```
  let s = String::from("test");
  let r1  = &s;
  let r2 = &s;
  println!("{r1} and {r2}");

  let r3 = &mut s;
```

r3 can be declared without any issues. This is due to the fact that the scope of r1 and r2 is limited to the last time they are used. The last use rule is followed.  
If the reference is not used at all the scope ends immediately after the mutable reference is created.

Note: Borrow errors are potential bugs the compiler is informing the developer about.

## Dangling References

In languages with pointers, you may encounter scenarios where the pointer points to memory that may have been deallocated or reallocated. This is completely avoideed in `safe` Rust.

In Rust the data a reference points to does not go out of scope or is deallocated until the reference is in scope.

Example of a dangling pointer

```
fn dangling_pointer() -> &String {
  let s = String::from("some string"); // This data would go out of scope at the end

  &s; // The pointer being returned would point to data that is reclaimed via drop.
} // The reference outlives the data in this case and thus an error would be thrown.
```

## The Slice Type
Slice lets you reference a contiguous sequence of elements in a collection. It is kind of a reference so has no ownership.


```
  let s = String::from("Hello World!");

  let r1 = &s[1..5];
  let r2 = &s[4..6];
```

With slices you have access to the state of the memory that is being referened.
For example lets say we have a string and we want to return the first word.

```
  let some_string = String::from("hello world");
  let bytes = some_string.as_bytes();

  for (i, &byt) in bytes.iter().enumerate() {
    if &byt == b' ' {
      return &some_string[..i] // return a slice from start to ith - 1 position.
    }
  }
```

Note that `&s[..i]` is equivalent to `&s[0..i]`. We could similarly have a slice from start to end of the contiguous sequence `&s[0..]` and a slice representing the entire string is simply `&s[..]`.

Slices tie the state of the contiguous sequence of elements. The compiler makes sure that the data being referenced stays the same to avoid issues.  

## String Slices

Rather than reference to the entire string. A string slice references a portion of the string. 
![Example of a String Slice](https://doc.rust-lang.org/book/img/trpl04-07.svg)

Note Your string slices must not be in the middle of a multibyte character positions. If they do not in the case of UTF-8 Strings. It will lead to an error.

Note &str is an immutable reference.

String literals are also of type `&str` since they point to specific points in the binary.

## String Slices as References

Using a string slice as a parameter in a function offers more flexibility. As you can pass string literals, slices from String types. This makes use of a concept of `Deref Coercions`.

## Other Slices

String slices are specific to &str. There are more general slices too.

Given an array such as

```
  let a = [1,2,3,4,5];
  let s1 = &a[..2];
```

The slice in the example above has type `&[i32]`. You can use this type of slice in all sorts of ways. Slices can work with various types of collections.
