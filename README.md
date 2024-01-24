# The pipe operator in Rust

This crate is exactly what you expect it to be, the pipe operator in Rust.
It's very simple, but effective nonetheless.

```rust
fn println(message: String) {
    println!("{message}");
}

fn greet(name: &'static str) -> String {
    format!("Hello, {name}!")
}

pipe!("David" // any expression
    |> greet
    |> println
);
```