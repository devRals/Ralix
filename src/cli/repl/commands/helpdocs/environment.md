# Environment

Environment is the place where all the variables being hold. When you
define variable using a `binding statement`, environment "allocates" some space
in the memory an gives you and `address`. All these happens in the `current scope`
that you're program currently in.

```ralix
int a = 10;
let b = 30;
```

## Scope

When you use a `block/scope expression` or call a `function` environment automatically
enters a new `scope`. Whenever you create a variable, function, type while you in
this newly entered scope, it holds these values for you. But when your job is done
with that scope or the call expression environment "drops" that scope. And so all
the variables it's holding.

```ralix
{
    int x = 10;
    // Do stuff with `x` here.
} // `x` is dropped right here and no longer available to use.

int y = x; // Error: x is not defined int this scope
```
