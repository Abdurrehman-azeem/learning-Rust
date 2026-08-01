# Cargo Build

To initiate a project that does not make use of cargo to one that makes use of cargo. Simply follow the instruction below.

-> Move the rust code to a src folder.
-> Then in the folder containing the src folder simply run `cargo init`

Follow the steps above and you have a cargo project.

With Cargo you can build the code by running the following command

```cargo build```

The above command generates an executable from the cargo code using the rust code. The build is stored in the [target/debug/] folder you can run it with rustc `./target/debug/[name_of_project]`.

In our case the following command will suffice:

`./target/debug/cargo_init_example`

Running `cargo build` also creates the `Cargo.lock` file which contains information of all the packages that were used to create the executable.

### Avoid The 2-step process

You can avoid the 2 step process listed above by making use of 

```
  cargo run
```

It builds and runs the executable and is used as a default. Also, if you try to build a cargo project without making changes to it `cargo run` would skip the build part. If you make changes, it will first compile your code then run it.

### Checking If your code is correct

What if you don't want to build your code (a slower process) but wish to check whether it is correct or not.

```
  cargo check
```

The above command checks whether the code has any issues and alerts you of them.

An example of an error in the code being detected.

```
error: unexpected closing delimiter: `}`
 --> src/main.rs:3:1
  |
1 | fn main() {
  |           - the nearest open delimiter
2 |     println!"Hello, World!");
  |                            - missing open `(` for this delimiter
3 | }
  | ^ unexpected closing delimiter
```

### Building For Release

Lets say you want to release your software for production. You want it to be optimized and as fast as possible.

```
  cargo build --release
```

The above command builds your code for release and stores it in target/release folder. This can take more time due to the optimizations.

Also good if you want to benchmark a version of your code.
