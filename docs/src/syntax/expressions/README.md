# Expressions

Expressions are constructs that produce a value.

## Literals

Literals are the most basic type of expression. They represent a fixed value in
the source code.

- **Integer literals**: `10`, `_`
- **Float literals**: `3.14`
- **Boolean literals**: `true`, `false`
- **Character literals**: `'a'`
- **String literals**: `"hello"`
- **Null literal**: `null`
- **Array literals**: `[1, 2, 3]`
- **Hash Map Literals**: `#{ "a": 1 , "b": 2 }`

## Identifiers

An identifier is a name that refers to a variable, constant, or function.

```c
int x = 5;
int y = x; // `x` is an identifier
```

## Prefix Expressions

A prefix expression has the operator before the operand.

- `-` (negation): `-10`
- `!` (logical NOT): `!true`
- `*` (dereference): `*ptr`
- `&` (address of): `&value`
- `~` (bitwise NOT): `~10`

## Infix Expressions

An infix expression has the operator between the operands.

- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `&&`, `||`
- Bitwise: `|`, `^`, `&`

```c
int x = 10;
int y = -4;

int sum = x + y;
bool are_equal = x == y;
int bit_or = x | y;
```

> [!NOTE]
> Every `int` and `float` values in ralix are 64-bits and this cannot be changed.
> Why? Because I suck

Ralix follow C-style precedence which look like this:

| Operator(s) | Precedence |
| ----------- | ---------- |
| Default expr parsing precedence | Lowest |
| `\|\|` | LogicalOr |
| `&&` | LogicalAnd |
| `\|` | BitwiseOr |
| `^` | BitwiseXOr |
| `&` | BitwiseAnd |
| `==`, `!=` | Equals |
| `>`, `<`, `>=`, `<=` | LessGreater |
| `>>`, `<<` | Shift |
| `+`, `-` | Sum |
| `*`, `/`, `%` | Product |
| `!`, `-`, `*`, `~` | Prefix |
| `func(param)` | FunctionCall |
| `hash_map["key"]`, `Namespace::Item`, `Class.attribute` | Access |

## `if` Expressions

An `if` expression allows for conditional execution. It must have an `else` block.

```c
str result =
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

An index expression is used to access an element of an array or a hash-map.

```c
arr[int] my_arr = [1, 2, 3];
int first = my_arr[0];
```

## Scope Expressions

A scope expression creates a new scope. The last expression in the scope is the
value of the scope expression.

```c
int y = {
    int x = 5;
    x + 1 // This is the return value of the scope
}; // x is dropped, y is 6
```

## `typeof` Expressions

A `typeof` expression returns the type of a value.

```py
int x = 10;
type[int] type_of_x = typeof x; // `type_of_x` is `type[int]`
```

> [!IMPORTANT]
> `typeof` expression only returns the type of the value during runtime.
> Example:

```rust
arr[int] my_arr = []; // Empty arrays automatically bind `unknown` type generic
typeof my_arr // `arr[unknown]`
```

## Function Literals

A function literal is an anonymous function. When you want to bind a function
you'd probably want to create a `function statement` instead

```c
let add = fn(int a, int b) -> int: {
    return a + b;
};
```
