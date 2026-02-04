# Syntax

Ralix has a simple and familiar syntax, inspired by languages like Rust and C. This section provides a reference for the language's syntax.

## Types

Ralix is a statically typed language, and it comes with a set of built-in types.

### Primitive Types

- **`int`**: A 64-bit signed integer.

  ```
  int x = 10;
  ```

- **`float`**: A 64-bit floating-point number.

  ```
  float y = 3.14;
  ```

- **`bool`**: A boolean value, which can be `true` or `false`.

  ```
  bool is_active = true;
  ```

- **`char`**: A single character.

  ```
  char initial = 'R';
  ```

- **`str`**: A string of characters.

  ```
  str name = "Ralix";
  ```

- **`null`**: A special type that has only one value, `null`. It is used to represent the absence of a value.

  ```
  int? a = null;
  ```

### Composite Types

- **`arr[T]`**: An array of elements of type `T`.

  ```
  arr[int] numbers = [1, 2, 3, 4, 5];
  ```

- **`fn(...) -> T`**: A function that takes a sequence of arguments and returns a value of type `T`.

  ```
  let add = fn(int a, int b) -> int: a + b;
  ```

- **`type[T]`**: A type that represents type `T` as a "type value"

  ```
  type[int] my_integer_ty = int;
  ```

> [!IMPORTANT]
> Let bindings auto binds the types

### Special Types

- **`type[T]`**: The type of a type. It is used to represent types as values.
- **`T*`**: A pointer to a value of type `T`.
- **`T?`**: A nullable type that can hold either a value of type `T` or `null`.
- **`void`**: A type that represents the absence of a value. It is used as the return type of functions that do not return a value.
- **`never`**: A type that represents a computation that never returns. It is used for functions that exit the program or run forever.
- **`unknown`**: A special type that is used by the type checker when it cannot determine the type of an expression.
