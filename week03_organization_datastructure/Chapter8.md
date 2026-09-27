# Common collections

The collections we're inspecting in this chapter are stored on the heap;
We’ll discuss three collections that are used very often in Rust
programs:
- A 'vector' allows you to store a variable number of values next to each other.
- A 'string' is a collection of characters.
- A 'hash map' allows you to associate a value with a particular key. \
It’s a particular implementation of the more general data structure called
a map.

## Vectors: storing list of values

ectors
allow you to store more than one value in a single data structure that puts
all the values next to each other in memory. \
Vectors can only store values of the same type.

They are useful when you have a list of items, such as the
lines of text in a file or the prices of items in a shopping cart.

### Creating a new vector

To create a new, empty vector, we can call the Vec::new function, as shown