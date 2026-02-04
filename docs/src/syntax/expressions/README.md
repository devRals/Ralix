# Expressions

Expressions are constructs that produce a value.

## Literals

Literals are the most basic type of expression. They represent a fixed value in the source code.

- **Integer literals**: `10`, `_`
- **Float literals**: `3.14`
- **Boolean literals**: `true`, `false`
- **Character literals**: `'a'`
- **String literals**: `"hello"`
- **Null literal**: `null`
- **Array literals**: `[1, 2, 3]`

## Identifiers

An identifier is a name that refers to a variable, constant, or function.

```rust
let x = 5;
let y = x; // `x` is an identifier
```

## Prefix Expressions

A prefix expression has the operator before the operand.

- `-` (negation): `-10`
- `!` (logical NOT): `!true`
- `*` (dereference): `*ptr`

## Infix Expressions

An infix expression has the operator between the operands.

- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `&&`, `||`

```c
int x = 10;
int y = -4;

int sum = x + y;
bool are_equal = x == y;
```

## `if` Expressions

An `if` expression allows for conditional execution. It must have an `else` block.

```c
let result =
    if x > 5: "greater"
    else: "not greater"
;
```

## Function Calls

A function call expression invokes a function with a list of arguments.

```rust
println("Hello, World!");
```

## Index Expressions

An index expression is used to access an element of an array.

```c
arr[int] my_arr = [1, 2, 3];
int first = my_arr[0];
```

## Scope Expressions

A scope expression creates a new scope. The last expression in the scope is the value of the scope expression.

```c
int y = {
    int x = 5;
    x + 1 // This is the return value of the scope
}; // x is dropped, y is 6
```

## `copy` Expressions

A `copy` expression is used to create a shallow copy of a value.

```rust
let a = [1, 2, 3];
let b = copy a;
```

## `typeof` Expressions

A `typeof` expression returns the type of a value.

```rust
let x = 10;
let type_of_x = typeof x; // `type_of_x` is `type[int]`
```

## `&` (Address Of) Expressions

The `&` operator creates a pointer to a value.

```c
int x = 10;
int* ptr_to_x = &x;
```

## Function Literals

A function literal is an anonymous function. When you want to bind a function
you'd probably want to create a `function statement` instead

```c
let add = fn(int a, int b) -> int: {
    return a + b;
};
```
