//_! strings - format! (can be used for String + String, &str + &str, String + &str), concat (can be used only with &str), String + &str (note String will be moved and no longer be available downstream)
//_! String, &str, &String as parameters to a function
//_! iter(), into_iter(), iter_mut()
//_! using structs with Iterator trait
//_! using closures as arguments to functions (with generics)
//_! using closures with structs
//_! generics with structs, enums, functions, methods
//todo closures as output type
//_ Vec<String> or [String]
//_! std::cmp::PartialOrd, std::ops::Add, std::ops::Mul
/_
enum Result<T,E> {
Ok(T),
Err(E),
}

enum Option<T> {
Some(T),
None,
}
_/
//_! Result | Option; match, unwrap, expect, ?
//_ ? can be used with Result that has std::io::Error, also with Option
//_ unwrap & expect can be used with Result that has std::io::Error

**Task**
Rust Generics 15m

## Generics can be used with...

1. Structs Fields & Methods
2. Enums
3. Functions Arguments & Output

## Traits can be added onto...

1. Argument Functions can accept
2. Methods Structs will implement
3. Implementation Block for Generic Structs

## Generics with Traits
