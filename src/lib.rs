/*!
A simple counter for debugging.

Basic usage of the crate is calling [`pass`], with each
call incrementing a static counter. The counter starts at 0, and
has a max of [`usize::MAX`].

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
pass(5999999);
```

You can pass a variable with the [`Display`] trait too:

```rust
let hw = String::from("Hello world");
let num = 5;
// Stderr: [<count>][<line_number] Hello world
pass!(hw);
// Stderr: [<count>][<line_number] 5
pass!(num);
```

[`Display`]: std::fmt::Display
*/

static mut COUNTER: usize = 0;

/**
Prints count information to stderr.

The macro takes an optional message argument, either
as a literal or a variable that has the [`Display`] trait.

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
pass(5999999);
```

You can pass a variable with the [`Display`] trait too:

```rust
let hw = String::from("Hello world");
let num = 5;
// Stderr: [<count>][<line_number] Hello world
pass!(hw);
// Stderr: [<count>][<line_number] 5
pass!(num);
```

[`Display`]: std::fmt::Display
*/
#[macro_export]
macro_rules! pass {
    () => {
        let n = increment();
        eprintln!("[Pass {}][Line {}]", n, line!());
    };

    ($mess:literal) => {
        let n = increment();
        eprintln!("[Pass {}][Line {}] {}", n, line!(), $mess);
    };

    ($mess:ident) => {
        let n = increment();
        eprintln!("[Pass {}][Line {}] {}", n, line!(), $mess);
    }
}

/**
Increment counter, return the result.

## Example

```rust
// Assuming the counter starts at 0
increment() // 1
increment() // 2
increment() // 3
```
*/
pub fn increment() -> usize {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

/**
Get the current counter number without incrementing.
The lowest number this function can return is 0.

## Example

```rust
let count1 = current_count();
let count2 = current_count();

assert_eq!(count1, count2);
```

*/
pub fn current_count() -> usize {
    unsafe { COUNTER }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pass_default() {
        pass!();
        pass!();
        pass!();
        pass!();
    }

    #[test]
    fn pass_message() {
        pass!("One");
        pass!("Two");
        pass!("Three");
        pass!("Four");
        pass!('4');
    }

    #[test]
    fn pass_ident() {
        let one = "one";
        let two = "two".to_string();
        let three = 3;

        pass!(one);
        pass!(two);
        pass!(three);
    }
}
