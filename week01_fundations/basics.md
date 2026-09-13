# Fundations

## Functions

"fn" defines a new function, followed by the function name "add2" and "-> i32" declares the return type as 32-bit signed integer

```rust
fn add2(x: i32, y: i32) -> i32 {
    // implicit return
    x + y
}
```

## Main() function 

### Variables - Bindings

Variables are called "Bindings" in Rust, and can be plain (unmutable) and mutable;

Mutable variables allows you to modify it in place:

```rust
let a = 1;
// a = 2; // error: `a` isn't mut
let mut b = 1;
b = 2; // ok
```

and also mutable reference allows you to modify it "remotely":

```rust
let mut b = 1;
let ref_b: &mut i32 = &mut b;
// notice reference itself doesn't need to be mut

*ref_b = 2; // same as previous example

fn add_1(x: &mut i32) {
    *x += 1; 
}

add_1(ref_b);
// by passing reference to b, other code can modify it
```

### Suffix for integers and float

This allows you to explicit the variable type:
m and you’l
```rust
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;
```

### Type inference

Most of the time the Rust compiler allow you to infere the expression type used to initialize a binding making its explicit declaration useless.

```rust
    let implicito_x = 1;
    let implicito_f = 1.3;
```

### Basic Arithmetics

```rust
    let somma = x + y + 13;
```

### Strings, Heap strings and println

String (&str) literals are defined as follows: 

```rust
    let x: &str = "Ciao mondo!";
```

We can use println to print it out toghether with the previously declared x: 

```rust
    println!("{} {}", f, x); // 1.3 Ciao mondo!
```

utimately a String can also be allocated on the heap: 

```rust
    let s: String = "Ciao mondo".to_string();
```

Main differences: 
- Literal: even if delcared as "mut" you can only reference to another literal and you cannot change the internal chars or increase/decrease its length (you must know the text content ahead of time before launching the program)
- String: if you declare a String as "mut" you can use many methods such as .push(), .push_str() or .truncate() and Rust will take care of increasing the allocated memory size automatically when needed.

### Slices 

A slice is an immutable view inside another string; 
A slice is an immutable cuple of pointers to the buffer of a string: it does not contain any character, just pointers to a static buffer or a buffer contained in another object (eg. we use the previously declared "s" in this case).

```rust
    let s_slice: &str = &s;

    println!("{} - {}", s, s_slice); // Ciao mondo - Ciao mondo
```

### Arrays and Vectors

We can declare a fixed-size array: 

```rust
    let quattro_int: [i32; 4] = [1, 2, 3, 4];
```

and we can declare a Vector (dynamic array<>)

```rust
    let mut vettore: Vec<i32> = vec![1, 2, 3, 4];
    vettore.push(5);
```

We can use Slices even in Vectors and Arrays using quite the same syntax as for the Strings:

```rust
    let slice: &[i32] = &vettore;
```

### Misc

Dugging prints using "{:?}": 

```rust
    println!("{:?} {:?}", vettore, slice); // [1, 2, 3, 4, 5] [1, 2, 3, 4, 5]
```

or defining a Tuple  (a sorted and fixed-size set of elements (possibilty having different types))

```rust
    let x: (i32, &str, f64) = (1, "ciao", 3.4);
```

We can also use "let" to destructure something: 

```rust
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c); // 1 ciao 3.4
```

and finally that's how indexing works: 

```rust
    println!("{}", x.1); // ciao
```

