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

### Invoking methods on the item in the pipeline

You can invoke methods on the item in the pipeline at any time by prefixing
the method identifier by a `.`

```rust
fn println(message: String) {
    println!("{message}");
}

// This is functionally the same as the "Partial invocation" example.
pipe!("Hello" 
    |> .to_uppercase
    |> println
);
```

### Closure based pipes

You can also use closures as pipes, so you don't have to define a
whole new function for every simple operation. 

Both types of closures are valid, you can have a closure that just
evaluates an expression, or you can have a whole code block.

```rust
pipe!("Hello"
    |> .to_uppercase
    |> |value| println!("{value}")
);
```