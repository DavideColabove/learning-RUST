# Chapter 3: Common programming concepts

## Variables  & Constants

'let' allows you do declare a variable which by default it's immutable: in order to make it mutable we need to add 'mut' while declaring it.

Constants on the other hand are so declared: 'const MAX_POINTS: u32 = 100_000;const MAX_POINTS: u32 = 100_000;'

## Shadowing 

As seen before in the guessing_game/ exercise we can declare a new variabile with the same name as a previuos variable, and the new variable shadows the previous.

We shadow a variable by using the same variable’s name and
repeating the use of the let keyword as follows:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```

This program first binds x to a value of 5 Then it shadows x by repeating
let x =, taking the original value and adding 1 so the value of x is then 6. The
third let statement also shadows x, multiplying the previous value by 2 to give
x a final value of 12.

Shadowing is different than marking a variable as mut, because we’ll get
a compile-time error if we accidentally try to reassign to this variable without
using the let keyword. By using let, we can perform a few transformations on
a value but have the variable be immutable after those transformations have
been completed.

The other difference between mut and shadowing is that because we’re
effectively creating a new variable when we use the let keyword again, we
can change the type of the value but reuse the same name. 

For example we might ask the user to input the amount of spaces they want between something, but we really need to store just a number:

```rust
let spaces = "     ";
let spaces = spaces.len();
```

This construct is allowed because the first spaces variable is a string type
and the second spaces variable, which is a brand-new variable that happens
to have the same name as the first one, is a number type.

However if we are trying to use 'mut' for that:
```rust
let mut spaces = "  ";
spaces = spaces.len();
```

We get the follwing error: "The error says we’re not allowed to mutate a variable’s type"

## Data types

There are two data types subsets:
- scalar
- compound

In case when many types are possible, such as when we converted a
String to a numeric type using parse in “Comparing the Guess to the Secret
Number” we must add a type annotation, like this:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don't add that the compiler will occour in to an error;

### Scalar Types 

A scalar type represents a single value. Rust has four primary scalar types:
integers, floating-point numbers, Booleans, and characters. You may recog­
nize these from other programming languages. 


#### Floating points
```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

#### Boolean 
```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

#### Chars
```rust
fn main() {
    let c = 'z';
    let z = 'Ƶ';
    let heart_eyed_cat = '😻';
}
```

### Compound Types

Compound types can group multiple values into one type. Rust has two
primitive compound types: tuples and arrays.

#### Tuple

A tuple is a general way of grouping together some number of other values
with a variety of types into one compound type.

We create a tuple by writing a comma-separated list of values inside
parentheses. Each position in the tuple has a type, and the types of the dif­
ferent values in the tuple don’t have to be the same. We’ve added optional
type annotations in this example:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

and we can then deconstruct this tuple by:
```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

In addition to destructuring through pattern matching, we can access
a tuple element directly by using a period (.) followed by the index of the
value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

#### Array
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather
than the heap or when you want to ensure you always have a fixed number of elements.

An example of good array usage is to map store the month's names (static, always 12 elements).

In order to access a specific element we use: 
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
```

When you attempt to access
an element using indexing, Rust will check that the index you’ve specified is
less than the array length. If the index is greater than the length, Rust will
panic, which is the term Rust uses when a program exits with an error.

This is the first example of Rust’s safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed.