# Using Modules to reuse and organize code

A module is a namespace that contains definitions of functions ortypes, and you can choose whether those definitions are visible outside their
module (public) or not (private). 

Here’s an overview of how modules work:
- The 'mod' keyword declares a new module. Code within the module
appears either immediately following this declaration within curly
brackets or in another file.
- By default, functions, types, constants, and modules are private. The
'pub' keyword makes an item public and therefore visible outside its
namespace.
- The 'use' keyword brings modules, or the definitions inside modules,
into scope so it’s easier to refer to them.

## mod and the Filesystem

We'll start our example by making a new project with Cargo using '--lib' which creates a library crate: a project that other people can pull into their projects as a dependency.

We’ll create a skeleton of a library that provides some general networking functionality; we’ll concentrate on the organization of the modules and
functions, but we won’t worry about what code goes in the function bodies.

In this case we created the 'communicator/' folder.

### Module definitions

We’ll first define a module named
network that contains the definition of a function called connect. Every module definition in Rust starts with the 'mod' keyword.

After the 'mod' keyword, we put the name of the module, network, and
then a block of code in curly brackets. Everything inside this block is inside
the namespace network. In this case, we have a single function named 'connect'.

We can also have multiple modules, side by side, in the same 'src/lib.rs'
file.

In our example now we have 'network::connect()' and 'client::connect()' which could have completely different functionalities, and names do not conflict because they're in two different modules.

We can also put modules inside of modules, which can be useful as your modules grow to keep related functionality organized together and separate functionality apart.

The client code and its 'connect' function might make more sense to users of our library if they were inside the network namespace instead. 

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

The functions 'network::connect()' and 'network::client::connect()'
are both named connect, but still they don’t conflict with each other because
they’re in different namespaces.

![lib_hierarchy](../img/lib_hierarchy.png)

### Moving modules to other files

We can use Rust’s module system along with multiple files to split up Rust projects so not everything lives in `src/lib.rs` or `src/main.rs`.

For example we might want to to separate the 'client', 'network',
and 'server' modules from `src/lib.rs` and place them into their own files.

We’re still declaring the client module inside the `src/lib.rs`, but by replacing the block with a semicolon, we’re telling Rust to look in another location for the code.

So we've now moved the content of the 'client' and 'network' modules content into their own files;

We still have to extract the 'server' module into its own file because it was a submodule of 'network'.

The error says we cannot declare a new module at this location and is
pointing to the 'mod server;' line in `src/network.rs`.

Instead of continuing to follow the same file-naming pattern we used
previously, we can do what the note suggests:
1. Make a new directory named 'network', the parent module’s name.
2. Move the `src/network.rs` file into the new 'network' directory and rename it `src/network.rs/mod.rs`.
3. Move the submodule file `src/server.rs` into the network directory.

Therefore, in order to extract a file for the 'network::client' submodule of
the 'network' module, we needed to create a directory for the 'network' module
instead of a `src/network.rs` file. \
The code that is in the network module then goes into the `src/network/mod.rs` file, and the submodule 'network::client' can have its own `src/network/client.rs` file. \
Now the top-level `src/client.rs` is unambiguously the code that belongs to the client module.

### Rules of Module filesystem

- If a module named 'foo' has no submodules, you should put the declarations for foo in a file named `foo.rs`.
- If a module named 'foo' does have submodules, you should put the declarations for foo in a file named `foo/mod.rs`.

This rules apply recursively, so if a module named 'foo' has a submodule
named 'bar' and 'bar' does not have submodules, you should have the following
files in your `/src` 

![foo_lib_hierarchy](../img/foo_lib_hierarchy.png)

The modules should be declared in their parent module’s file using the
mod keyword.

## Controlling visibility with pub

The '/communicator' project we've built does compile with 'cargo build' but we still get warnings saying that the 'client::connect()', 'network::connect()' and 'network::server::connect()' functions are not being used.

In order to understand what's happeing we're going to create a new `/src/main.rs` file invoking the 'client::connect()' functions inside the main function.

However, invoking cargo build will now give us an error
after the warnings: `error[E0603]: module 'client' is private`.

The default state of all code in Rust is private: no one else is allowed to use the code. \
If you don’t use a private function within your program, because your program is the only code allowed to
use that function, Rust will warn you that the function has gone unused.\
After you specify that a function such as `client::connect()` is public, not
only will your call to that function from your binary crate be allowed, but also the warning that the function is unused will go away.

### Making a function public

To tell Rust to make a function public, we add the 'pub' keyword to the
start of the declaration.

We can add this attribute both to modules and functions!

Summarizing privacy rules:
- If an item is public, it can be accessed through any of its parent modules.
- If an item is private, it can be accessed only by its immediate parent
module and any of the parent’s child modules.