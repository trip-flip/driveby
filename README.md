# DriveBy

Ever need to a counter to see how many times a line of code
is passed. Here you go.

## Examples

Run without arguments to get basic debug information:

```rust
pass!(); // Stderr: [<count>][<line_number>]
```

You can pass a literal to have it printed:

```rust
// Stderr: [<count>][<line_number] Special message
pass!("Special message");
// Stderr: [<count>][<line_number] a
pass!('a');
// Stderr: [<count>][<line_number] 5999999
pass(5999999)
```

You can pass a variable with the `Display` trait too:

```rust
let hw = String::from("Hello world");
let num = 5;
// Stderr: [<count>][<line_number] Hello world
pass!(hw);
// Stderr: [<count>][<line_number] 5
pass!(num);
```