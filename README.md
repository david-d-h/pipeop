# The pipe operator in Rust

This crate is exactly what you expect it to be, the pipe operator in Rust.
It's very simple, but effective nonetheless.

## Usage

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

### Partial invocation of pipes

You can partially invoke pipes, the `@` token will be replaced by the value
currently going through the pipeline. The `@` token can be in any position, 
not just at the start or end.

```rust
fn println(message: &'static str, upcase: bool) {
    let message = match upcase {
        true => message.to_uppercase(),
        false => message.to_string(),
    };

    println!("{message}");
}

pipe!("Hello" |> println(@, true)); // will output "HELLO" to stdout
```