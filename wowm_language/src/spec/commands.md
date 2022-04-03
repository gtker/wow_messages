# Commands

The following commands have compiler behavior defined.


## `tag_all`

Applies the selected tag to all objects in the file.

### Format

```ignore
#tag_all <tag_name> "<tag_value>";
```

Where `<tag_name>` is the name of the tag to be applied and `<tag_value>` is the value.

### Example

```ignore
#tag_all versions "1.12";

enum Boolean : u8 {
    YES = 0;
    NO = 1;
}

struct S {
    u8 basic;
}
```

would be the same as

```ignore
enum Boolean : u8 {
    YES = 0;
    NO = 1;
} {
    versions = "1.12";
}

struct S {
    u8 basic;
} {
    versions = "1.12";
}
```
