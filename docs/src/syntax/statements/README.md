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

The `const` statement is used to create a new constant binding. Constants are
immutable and must have their type specified.

```c
// Create a constant `PI` of type `float`
const float PI = 3.14159;
```

## Type Alias Statements

Using the type keyword you can create your own type aliases. Once a type alias defined
it cannot be changed afterwards.

```c
type MyStr = str?;
MyStr my_value = "hehe! I'm in danger!"; // Don't judge. This line came
                                         // to my mind for no reason
```

You also can use the types you got from the `typeof` expression. And also
use them in binding statements.

```rust
const float PI = 3.14159;
let MyFloat = typeof PI;
MyFloat my_type_is_same_as_PI = 1.2;
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

An expression statement is an expression that is followed by a semicolon.
The value of the expression is discarded unless it's the last expression
statement that has been evaluated. This can be useful when using scope expressions.

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
items["b"] = [6,7,8];

arr[float] nums = [1.0, 2.7, 3.3, float(4)];
nums[2] = 5.8;
```

> [!IMPORTANT]
> Note that index assignment operations can only update _existing values_.
> If you wanna add a new value to a new hash-map using a key that hash-map
> isn't using this operation will simply do nothing
