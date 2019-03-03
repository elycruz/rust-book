# rust-book
Rust book examples.

## Notable notes:
Chp3 - Chp5:
Reference names for reserved keyword names in code symbols;  E.g.,
```
let r#len = 99; // lets us use `len` as a var name even though it's a reserved keyword

// Slicing
&some_string[..9] // Turns `String` into slice from `0` to `9`
&some_string[9..] // "" "" "" from 9 to length of String
&some_string[..] // Turns `String` into slice
some_string.as_str() // Turns `String` into slice
```
