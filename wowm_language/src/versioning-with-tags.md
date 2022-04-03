# Versioning through tags

When resolving type names of dependencies the tag system is used.
The compiler will accept names that are as specific or less specific than the tag on the object that needs a dependency.

So in the example:
```rust,ignore
struct User {
    Boolean has_something;
} {
    versions = "1.12";
}

enum Boolean : u8 {
    FALSE = 0;
    TRUE = 1;
} {
    versions = "1";
}
```

The `struct` `User` is allowed to use the `enum` `Boolean` because `Boolean` has a less specific `versions` tag than `User` does.
That is, `User` is valid for all versions of `1.12`, while `Boolean` is valid for all versions of `1`.
It would not be possible to be valid for all versions of `1` but not `1.12` specifically.

There can not be two objects with the same name and the same version.
Two different versions can be each be valid for their own version, but it does not make sense for the same object to have two different representations for the same version.

So this is allowed:
```rust,ignore
enum Boolean : u8 {  /* Name of Boolean */
    FALSE = 0;
    TRUE = 1;
} {
    versions = "1"; /* <-- versions 1 here */
}

enum Boolean : u8 { /* <-- Same name */
    FALSE = 0;
    TRUE = 1;
    MAYBE = 2;
} {
    versions = "2" /* <-- versions 2 here */
}
```

But this is not:
```rust,ignore
enum Boolean : u8 {
    FALSE = 0;
    TRUE = 1;
} {
    versions = "1 2"; /* <-- Added 2 here */
}

enum Boolean : u8 {
    FALSE = 0;
    TRUE = 1;
    MAYBE = 2;
} {
    versions = "2" /* <-- Conflicts with this versions */
}
```

# Mass versioning

The `#tag_all` command can be used to tag all objects in a file with the same `versions` string.

```rust,ignore
##tag_all versions "1.11 1.12"

enum Boolean : u8 {
    FALSE = 0;
    TRUE = 1;
} /* <-- Will have "versions" = "1.11 1.12" automatically applied */


struct Thing {
    Boolean is_thing;
    u8 basic;
} /* <-- Will have "versions" = "1.11 1.12" automatically applied */
```
