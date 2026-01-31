# Types

In `ralix` every value has a type. Which are suitable for working in harmony with
each other, they exist to control it. Even types has their own types!
Yes, I know it sounds confusing but now we'll deep in to it. But first we recommend
you to first check this table that shows the `builtin types` in `ralix`:

| Type           | Example            | Explanation                            |
| -------------- | ------------------ | -------------------------------------- |
| int            | `10`               | Reperesents the integer values         |
| float          | `3.14159`          | Represents the floating point values   |
| bool           | `true`             | Has only two states `true`, `false`    |
| char           | `'R'`              | UTF-8 character                        |
| str            | `"meow :3"`        | A slice of `char`s. a simple text      |
| null           | `null`             | Uses when the value is not spesified   |
| type           | `int`              | A type as value                        |
| `ty`?          | `null`             | Nullable: Either null or the type `ty` |
| `ty`\*         | `<0x5644888b5430>` | The address of the `ty` typed value    |
| fn(`ty`)->`ty` | `fn(int a)->int:a` | A function takes a param and returns   |

## Types as Values

You can use types as values in `ralix`. You can get the type of the value using the
`typeof` expression like this:

```ralix
int a = 10;
type b = typeof a; // gives you int in type `type`
```

> [!IMPORTANT]
> `typeof` expression doesn't work on values that has type `<ty>?` (Nullable)

You can use "type values" when trying to match a values type and
do a specific operation for the matched type.

> [!NOTE]
> We're still working on project `ralix`. In the future updates it's possible
> that we might add some more types and more flexibility for them.
