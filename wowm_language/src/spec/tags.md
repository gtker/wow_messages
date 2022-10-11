# Tags

<!-- toc -->

The following tags have compiler defined meaning.

## For Objects, Definer Enumerators, and Object Declarations

### `description`

Used to describe the general purpose of something in a high level way.
Will be shown in documentation and generated code.

Compared to `comment` this is more general and high level.

The format is general text input.

For example

```rust,ignore
smsg SMSG_TEST = 0x00 {
    u8 basic;
} {
    description = "Sent every time the client tests.";
}
```

Multiple `description` tags are allowed and will not overwrite each other.
One `description` tag should be used for each distinctive subject.

For example

```rust,ignore
smsg SMSG_TEST = 0x00 {
    u8 basic;
} {
    description = "Sent every time the client tests.";
    description = "Testing occurs whenever [...]";
}
```

#### Linking

Text in comments and descriptions can hyperlink to other objects by surrounding them in square brackets (`[` and `]`).

For example

```rust,ignore
smsg SMSG_TEST = 0x00 {
    u8 basic;
} {
    description = "Reponse to [CMSG_TEST].";
}
```

### `comment`

Used to describe quirks or non-obvious information regarding something.
Will be shown in documentation and generated code.

Compared to `description` this is more specific and lower level.

The format is general text input.

For example

```rust,ignore
smsg SMSG_TEST = 0x00 {
    u8 basic;
} {
    comment = "Spell id 1337 prevents this from being sent.";
}
```

Multiple `comment` tags are allowed and will not overwrite each other.
One `comment` tag should be used for each distinctive comment.

For example

```rust,ignore
smsg SMSG_TEST = 0x00 {
    u8 basic {
        comment = "This is one thing to say.";
        comment = "This is something else.";
    }
}
```

#### Linking

Text in comments and descriptions can hyperlink to other objects by surrounding them in square brackets (`[` and `]`).

For example

```rust,ignore
smsg SMSG_TEST = 0x00 {
    u8 basic;
} {
    description = "Reponse to [CMSG_TEST].";
}
```

## For Objects

### `versions`

Used for defining world message versions.

MUST be in the format of either:

* `<major>`
* `<major>.<minor>`
* `<major>.<minor>.<patch>`
* `<major>.<minor>.<patch>.<build>`
* `*`

Or any repetition of any of the above separated by space, except for `*` which overrides all other options.

Where:

* `<major>` is the expansion release of the game. E.g. 1 for Vanilla, 2 for The Burning Crusade.
* `<minor>` is the major patch release of the game, sometimes named.
* `<patch>` is the quick patch release of the game.
* `<build>` is the exact build number of the client. Often different between localizations.
* `*` means all known versions.

Multiple versions separated by a space are allowed.

For example

```rust,ignore
struct S {
    u8 basic;
} {
    versions = "1.12";
}
```

Or covering more versions

```rust,ignore
struct S {
    u8 basic;
} {
    versions = "1.12 1.11 1.8";
}
```

Which cover versions `1.12`, `1.11`, and `1.8` and no others.

### `paste_versions`

Used for defining versions but objects that have an identical layout,
but where the types inside the object have different versions so that it is not the same object.

This is intended to reduce copy pasting for objects with identical layouts across versions.
Format is the same as `versions`.

For example

```rust,ignore
enum Complex : u8 {
    ONE = 1;
    TWO = 2;
} {
    versions = "1.12";
}

enum Complex : u8 {
    ONE = 1;
    TWO = 2;
    THREE = 3;
} {
    versions = "2.4.3";
}

struct S {
    Complex c;
} {
    paste_versions = "1.12 2.4.3"
}
```

would be the same as

```rust,ignore
enum Complex : u8 {
    ONE = 1;
    TWO = 2;
} {
    versions = "1.12";
}

enum Complex : u8 {
    ONE = 1;
    TWO = 2;
    THREE = 3;
} {
    versions = "2.4.3";
}

struct S {
    Complex c;
} {
    versions = "1.12"
}

struct S {
    Complex c;
} {
    versions = "2.4.3"
}
```

### `login_versions`

Used for defining the protocol version used when clients initially connect.

MUST be in the format of a single positive integer.

This value is first sent by the client in `CMD_AUTH_LOGON_CHALLENGE_Client` or `CMD_AUTH_RECONNECT_CHALLENGE_Client`.

Multiple versions separated by a space are allowed.

For example

```rust,ignore
struct S {
    u8 basic;
} {
    login_versions = "2";
}
```

Or covering more versions

```rust,ignore
struct S {
    u8 basic;
} {
    login_versions = "2 3 8";
}
```

Which means that `S` is valid for versions 2, 3, and 8, and no other versions.

### `test`

Used to signify that the object is made up for internal testing purposes.
Objects with these should be ignored unless testing.

Allowed values are either `true` or `false`.
The absence of a `test` tag is the same as `test = "false";`.

For example

```rust,ignore
smsg SMSG_TEST = 0x00 {
    u8 basic;
} {
    test = "true";
}
```

## For Definer Enumerators

### `display`

Used for defining how the enumerator should be displayed to users.
Can be used when the "real" name contains characters that aren't allowed in enumerators, or the enumerator name is
undescriptive.

For example

```rust,ignore
enum Map : u8 {
    VASHJIR = 0 {
        display = "Vashj'ir";
    }
}
```

## For Container Declarations

### Valid Values

Specifies valid values for a integer type.
Any values that do not conform to the values defined will lead to an invalid message which should be discarded.
It is a good idea to use a `comment` tag to explain why the values are invalid.

Note that values outside of the `valid` range **MUST** be invalid in every situation, either because the client can't
handle values outside of the valid range, or if regular clients can never send values outside of the valid range.

#### `minimum_valid_value`

Specifies the lowest value that is valid.

MUST be in the format of a single integer.

For example

```rust,ignore
struct S {
    u8 basic {
        minimum_valid_value = "1";
        comment = "Can never be zero because of [...]";
    }
}
```

#### `maximum_valid_value`

Specifies the highest value that is valid.
MUST be in the format of a single integer.

For example

```rust,ignore
struct S {
    u8 basic {
        maximum_valid_value = "10";
        comment = "Values above 10 crash the client";
    }
}
```

#### `valid_range`

Specifies a range of valid values.
MUST be in the format of two integers separated by a space.

Note that these values are _inclusive_, so for a `valid range = "2 10";` both 2 and 10 are also valid values while 1 and
11 are not.

For example

```rust,ignore
struct S {
    u8 basic {
        valid_range = "0 10";
        comment = "Values outside of 0-10 crash the client";
    }
}
```

#### `valid_values`

Specifies the only values which are valid.
MUST be in the format of one or more integers separated by a space.

For example

```rust,ignore
struct S {
    u8 basic {
        valid_values = "2 4 6 8 10";
        comment = "Odd values crash the client.";
    }
}
```

