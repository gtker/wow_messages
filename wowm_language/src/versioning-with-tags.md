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

# World and Login versions

`clogin` and `slogin` messages are used for communication with the realm/authentication server.
These can only have `login_versions`.

`cmsg`, `smsg`, and `msg` messages are used for communication with the world/game server.
These can only have `versions`.

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

# Paste versions

Versions must fully deliver what they promise.
What this means is that every single field used must also cover at least the same version as the containing object.
For example, if there is a `Race` enum that has been determined to be valid for `1` (Vanilla) and another `Race` enum that has determined to be valid for `2` (The Burning Crusade), it is not valid to create a single message that uses the `Race` enum and covers both `1` and `2`.
An object that only contains a vanilla `Race` and an object that only contains a Burning Crusade `Race` are not identical, despite the fields taking up the same amount of space (1 byte, a `u8`).

So the following is **NOT VALID**:
```rust,ignore
enum Race : u8 {
    HUMAN = 1;
    ORC = 2;
    DWARF = 3;
    NIGHT_ELF = 4;
    UNDEAD = 5;
    TAUREN = 6;
    GNOME = 7;
    TROLL = 8;
    GOBLIN = 9;
} {
    versions = "1"; /* <-- Only covers 1 */
}

enum Race : u8 {
    HUMAN = 1;
    ORC = 2;
    DWARF = 3;
    NIGHT_ELF = 4;
    UNDEAD = 5;
    TAUREN = 6;
    GNOME = 7;
    TROLL = 8;
    GOBLIN = 9;
    BLOOD_ELF = 10; /* <-- Clearly not the same */
    DRAENEI = 11;
    FEL_ORC = 12;
    NAGA = 13;
    BROKEN = 14;
    SKELETON = 15;
    VRYKUL = 16;
    TUSKARR = 17;
    FOREST_TROLL = 18;
} {
    versions = "2"; /* <-- Only covers 2 */
}

cmsg CMSG_INVALID_MADE_UP_MSG = 0xDEADBEEF1471D {
    Race race;
} {
    versions = "1 2"; /* <-- NOT VALID. Neither enum can cover both versions 1 and 2. */
}
```

There are many messages where the only difference between expansions is that they update an enum that is specific to that expansion and version.
For example `CMSG_EMOTE`:

```rust,ignore
cmsg CMSG_EMOTE = 0x0102 {
    Emote emote; /* <-- This Emote only covers 1.12 */
} {
    versions = "1.12";
}

cmsg CMSG_EMOTE = 0x0102 {
    Emote emote; /* <-- This Emote only covers 2.4.3 */
} {
    versions = "2.4.3";
}

cmsg CMSG_EMOTE = 0x0102 {
    Emote emote; /* <-- This Emote only covers 3.3.5 */
} {
    versions = "3.3.5";
}
```

There can not be a single message that is valid for all 3 versions since the `Emote`s are different.

Instead `paste_versions` can be used.
This effectively copy and pastes the textual `wowm` representation and creates 3 different messages that are each only valid for one version.
The two examples above and below are effectively the same, while the bottom one has significantly less duplication.
Comments, descriptions, and other tags are also duplicated for the different versions.

```rust,ignore
cmsg CMSG_EMOTE = 0x0102 {
    Emote emote;
} {
    paste_versions = "1.12 2.4.3 3.3.5";
}
```

`paste_versions` only exists for World versions (`versions`), not for Login versions (`login_versions`).