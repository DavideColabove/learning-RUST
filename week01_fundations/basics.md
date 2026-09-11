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


