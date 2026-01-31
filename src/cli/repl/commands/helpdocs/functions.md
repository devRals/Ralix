# Functions

In `ralix` functions are "expressions" which means functions are can be used
as _values_. You can define a set of code in a function and then later on
you can use them anytime you want. You can define a function simply using a simple
`let binding statement` like this:

> [!IMPORTANT]
> When defining a function you have specify the type of parameters and
> the return type it's returns something other than type `void`

```ralix
let add_ten = fn(int x) -> int: // A function that takes a value in `int` type
    x + 10                      // and gives you another value in `int` type
```

But we don't recommend you to define a function like this. Instead you can use the
`function statement` sugar syntax which does absolutely the same thing.

```ralix
fn add_ten(int x) -> int: x + 10
```

## Block Expressions

Sometimes just a single expressions is not enough when you want to create calculations
for a function. Then it's the time when you should use block expressions.
Here's an example of function definition that's using a block expression:

> [!IMPORTANT]
> Block expressions ALWAYS returns the last expression you give it. With
> semicolon ";" or not. It doesn't matter.

```ralix
fn expensive_calculation(int x) -> int: {
    int two_times_x = x * 2;
    two_times_x = x / 2;
    // Use your imagination. I'm so dumb to create my own long function here c:

    x * 30;
}
```

> You should know when you create a block expressions the `environment` enters a
> new `scope`. You can learn what those are by using "/help environment" command.

## Call Expressions

When you define a function and you need to use it this is where you should
use a `call expression`. It's important that you need to use same amount of
values as parameters for the defined function in the call expression.

```ralix
let my_variable = add_ten(4); // Gives you the value "14"
```
