# Statements

Statements are instructions that perform an action. In Ralix, statements only .

## Binding statement

The binding statement is used to create a new variable binding. You can specify the
type of the value. If you want it's type to be specified automatically you can
use the `let` statements

```c
// Create a variable `x` of type `int` with a value of 5
int x = 5;

// Create a variable `y` and let the compiler infer its type
str y = "hello";

// Auto binds the type `float`
let pi = 3.14;
```

## `const` statement

The `const` statement is used to create a new constant binding. Constants are immutable and must have their type specified.

```c
// Create a constant `PI` of type `float`
const float PI = 3.14159;
```

## `fn` statement

The `fn` statement is used to create a new function. Functions can have parameters
and a return type. Optionally you can also bind `const fn` statements.
This is allowed because functions are just binding statements that the value is just
a regular function.

```c
// Create a function `add` that takes two integers and returns an integer
fn add(int a, int b) -> int: a + b;
```

For flexibility you can use "type generics" in function parameters and return types.

```rust
fn first[T](arr[T] x) -> T? : x[0]
```

## `return` statement

The `return` statement is used to exit a function and optionally return a value.

```c
fn get_greeting() -> str: {
    return "Hello, Ralix!";
}
```

## Expression statement

An expression statement is an expression that is followed by a semicolon. The value of the expression is discarded.

```c
// The function call is an expression statement
println("Hello, World!");
```

## Assignment

An assignment statement is used to change the value of an existing variable.

```go
int x = 5;
x = 10;

map[str, arr[int]] items = #{ "a": [0,1,2], "b": [3,4,5] };
items["c"] = [6,7,8];
```
