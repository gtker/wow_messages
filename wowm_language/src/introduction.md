# Introduction

Wowm is a domain specific language for describing the layout of messages used in the Client/Server communication for World of Warcraft with the intention of auto generating documentation and usable programming libraries.

The language leans heavily on syntax from C/C++ for declaring _definers_, _built-in types_, and _containers_.
Definers are simple wrappers over regular numbers that name valid values.
Containers specify the order and layout of messages.

Definers can either be `flag`s or `enum`s.
`enum`s can only have one valid named value, while `flag`s can have multiple.
`flag`s are also known as [bitfields](https://en.wikipedia.org/wiki/Bit_field).

The following is an example enum and flag:
```rust,ignore
enum TestEnum : u8 {
    TEST_ONE = 1;
    TEST_TWO = 2;
    TEST_THREE = 3;
}
/* TestEnums can only be either 1, 2, or 3. */


flag TestFlag : u8 {
    TEST_ONE = 1;
    TEST_TWO = 2;
    TEST_FOUR = 4;
}
/* TestFlags can be either 1, 2, 4 or any combination thereof like 0, 3, 7, or 5. */
```
The `u8` after the `:` specifies the basic type of the definers.
This is the type that is actually sent over the network.
A `u8` is an unsigned integer of 8 bits (1 byte).
A `u32` would be an unsigned **little endian** integer of 32 bits (4 bytes).
A `u32_be` would be an unsigned **big endian** integer of 32 bits (4 bytes).

For a full list of built-in types, see the [built-in types section in the specification](spec/lang-spec.md#built-in-types).

Containers specify the actual message as well as `struct`s which can be used as a type in other containers.

A full list of container variants can be found in [the container section of the specification](spec/lang-spec.md#container).

The following is an example of a `struct` using built-in types, that is itself also used as a type in an `smsg` (world message sent from the server):
```rust,ignore
struct TestStruct {
    u32 price_in_gold;
    u32 amount_purchased;
}

/* 
    Number after "=" is the opcode. Use hexadecimal.
    Size is automatically calculated based on contents.
*/
smsg SMSG_TEST = 0xFF {
    CString name;
    TestStruct t;
}
/* The above is functionally identical to */
smsg SMSG_TEST = 0xFF {
    CString name;
    u32 price_in_gold;
    u32 amount_purchased;
}
```

All objects must specify which version of the game they work with.
See the [section on versioning with tags](versioning-with-tags.md)
